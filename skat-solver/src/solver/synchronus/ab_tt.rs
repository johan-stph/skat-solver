use std::cmp::{max, min};
use std::collections::HashMap;
use fxhash::FxHashMap;
use crate::solver::{GlobalState, Player};
use crate::solver::synchronus::ab_tt::Bounds::{LowerBound, UpperBound, Valid};
use crate::solver::synchronus::local_state::LState;


pub enum Bounds {
    Valid,
    LowerBound,
    UpperBound
}



pub struct EnhancedSolver {
    pub global_state: GlobalState,
    pub look_up_table: FxHashMap<(u32), (i8, Bounds)>
}

impl EnhancedSolver {
    fn try_insert(&mut self, local_state: &LState, score: i8, bound: Bounds) {
        if local_state.current_suit.is_none() {
            self.look_up_table.insert(local_state.get_hash_better(&self.global_state), (score, bound));
        }
    }
    pub fn ab_tt(&mut self, local_state: LState, agoof: i8, bgoof: i8) -> i8 {
        if local_state.is_terminal() {
            return 0;
        }
        let mut new_alpha = agoof;
        let mut new_beta = bgoof;

        if local_state.current_suit.is_none() {
            if let Some(result) = self.look_up_table.get(&local_state.get_hash_better(&self.global_state)) {
                match result.1 {
                    Valid => {
                        return max(0, result.0)
                    }
                    LowerBound => {
                        new_alpha = max(new_alpha, result.0)
                    }
                    UpperBound => {
                        new_beta = min(new_beta, result.0)
                    }
                }
                if new_alpha >= new_beta {
                    return result.0
                }
            }
        }
        let mut result;
        if local_state.is_max_node(&self.global_state) {
            result = new_alpha;
        } else {
            result = new_beta;
        }

        for next_state in local_state.get_next_states(&self.global_state) {
            let t_q = next_state.achieved_points as i8;
            if local_state.is_max_node(&self.global_state) {
                let succ_val= t_q +
                    self.ab_tt(next_state, result - t_q, new_beta - t_q);
                result = max(result, succ_val);
                if result >= new_beta {
                    self.try_insert(&local_state, result, LowerBound);
                    return result
                }
            } else {
                let succ_val = t_q + self.ab_tt(next_state, new_alpha - t_q, result - t_q);
                result = min(result, succ_val);
                if result <= new_alpha {
                    self.try_insert(&local_state, result, UpperBound);
                    return result
                }
            }
        }
        self.try_insert(&local_state, result, Valid);
        result
    }
}


pub struct Solver {
    pub global_state: GlobalState,
    pub look_up_table: HashMap<(u32, Player), (i32, Bounds)>
}


impl Solver {
    fn try_insert(&mut self, local_state: &LState, score: i32, bound: Bounds) {
        if local_state.current_suit.is_none() {
            self.look_up_table.insert(local_state.get_hash(), (score, bound));
        }
    }
    pub fn ab_tt(&mut self, local_state: LState, agoof: i32, bgoof: i32) -> i32 {
        if local_state.is_terminal() {
            return 0;
        }
        let mut new_alpha = agoof;
        let mut new_beta = bgoof;

        if local_state.current_suit.is_none() {
            if let Some(result) = self.look_up_table.get(&local_state.get_hash()) {
                match result.1 {
                    Valid => {
                        return max(0, min(120, result.0))
                    }
                    LowerBound => {
                        new_alpha = max(new_alpha, result.0)
                    }
                    UpperBound => {
                        new_beta = min(new_beta, result.0)
                    }
                }
                if new_alpha >= new_beta {
                    return result.0
                }
            }
        }
        let mut result;
        if local_state.is_max_node(&self.global_state) {
            result = new_alpha;
        } else {
            result = new_beta;
        }

        for next_state in local_state.get_next_states(&self.global_state) {
            let t_q = next_state.achieved_points as i32;
            if local_state.is_max_node(&self.global_state) {
                let succ_val= t_q +
                    self.ab_tt(next_state, result - t_q, new_beta - t_q);
                result = max(result, succ_val);
                if result >= new_beta {
                    self.try_insert(&local_state, result, LowerBound);
                    return result
                }
            } else {
                let succ_val = t_q + self.ab_tt(next_state, new_alpha - t_q, result - t_q);
                result = min(result, succ_val);
                if result <= new_alpha {
                    self.try_insert(&local_state, result, UpperBound);
                    return result
                }
            }
        }
        self.try_insert(&local_state, result, Valid);
        result
    }


    pub fn minimax_with_alpha_beta_tt(&mut self, local_state: LState, alpha: i8, beta: i8) -> (i8, Option<LState>) {
        if local_state.is_terminal() {
            return (0, None);
        }
        let mut new_alpha = alpha;
        let mut new_beta = beta;

        if local_state.current_suit.is_none() {
            if let Some(result) = self.look_up_table.get(&local_state.get_hash()) {
                if let Valid = result.1 {
                    return (result.0 as i8, None)
                }
            }
        };

        let mut optimal_move: Option<LState> = None;

        for next_state in local_state.get_next_states(&self.global_state) {
            let achieved = next_state.achieved_points as i8;
            let poss_alpha_or_beta = achieved + self.minimax_with_alpha_beta_tt(next_state,
                                                                                new_alpha - achieved,
                                                                                new_beta - achieved).0;
            if local_state.is_max_node(&self.global_state) {
                if poss_alpha_or_beta > new_alpha {
                    new_alpha = poss_alpha_or_beta;
                    optimal_move = Some(next_state);
                }
            } else {
                if poss_alpha_or_beta < new_beta {
                    new_beta = poss_alpha_or_beta;
                    optimal_move = Some(next_state)
                }
            }
            if new_alpha >= new_beta {
                break
            }

        }
        if local_state.is_max_node(&self.global_state) {
            self.try_insert(&local_state, new_alpha as i32, Valid);
            (new_alpha, optimal_move)
        } else {
            self.try_insert(&local_state, new_beta as i32, Valid);
            (new_beta, optimal_move)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::solver::bitboard::{BitCards, EMPTY_CARD, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN};
    use crate::solver::{GlobalState, Player, Variant};
    use crate::solver::synchronus::ab_tt::Solver;
    use crate::solver::synchronus::local_state::LState;

    #[test]
    fn minmax_v2_paper_one_subset() {
        let player_one = KREUZ_JACK | KREUZ_TEN;
        let player_two = PIQUS_JACK | HEARTS_JACK;
        let player_three = KREUZ_QUEEN | KREUZ_EIGHT;
        let all_cards = player_one | player_two | player_three;
        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };
        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        //let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        //assert_eq!(result.0, 4);

        let player_one = KREUZ_JACK | KREUZ_TEN | HEARTS_TEN;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS;
        let all_cards = player_one | player_two | player_three;
        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };
        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        assert_eq!(result.0, 7);

        let player_one = KREUZ_JACK | KREUZ_TEN | HEARTS_TEN | PIQUS_KING;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT | KARO_EIGHT;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | PIQUS_EIGHT;
        let all_cards = player_one | player_two | player_three;
        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        assert_eq!(7, result.0)
    }
    #[test]
    fn minmax_v2_paper_one_5() {
        let player_one = HEARTS_TEN | HEARTS_KING;
        let player_two = KARO_ASS | KARO_TEN;
        let player_three = HEARTS_ASS | PIQUS_NINE;
        let all_cards = player_one | player_two | player_three;
        assert_eq!(all_cards.0.count_ones(), 6);

        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };
        //let res_alt = minimax_with_alpha_beta(local_state, &global_state, 0, 120);

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        assert_eq!(result.0 + 7, 7);
    }

    #[test]
    fn minmax_v2_paper_one_4() {
        let player_one = KREUZ_JACK | HEARTS_TEN | HEARTS_KING;
        let player_two = PIQUS_JACK | KARO_ASS | KARO_TEN;
        let player_three = KREUZ_QUEEN |  HEARTS_ASS | PIQUS_NINE;
        let all_cards = player_one | player_two | player_three;
        assert_eq!(all_cards.0.count_ones(), 9);

        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::Two,
            current_suit: None,
            achieved_points: 0,
        };
        //let res_alt = minimax_with_alpha_beta(local_state, &global_state, 0, 120);

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        assert_eq!(result.0, 7);
    }

    #[test]
    fn minmax_v2_paper_one_3() {
        let player_one = KREUZ_JACK
            | HEARTS_TEN | HEARTS_KING | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | KARO_ASS | KARO_TEN | KARO_QUEEN;
        let player_three = KREUZ_QUEEN |  HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE;
        let all_cards = player_one | player_two | player_three;
        assert_eq!(all_cards.0.count_ones(), 12);

        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::Two,
            current_suit: None,
            achieved_points: 0,
        };

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);

        assert_eq!(result.0, 7);
    }

    #[test]
    fn minmax_v2_paper_one_2() {
        let player_one = KREUZ_JACK
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN |  HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE |  KARO_SEVEN;
        let all_cards = player_one | player_two | player_three;
        assert_eq!(all_cards.0.count_ones(), 15);

        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::Two,
            current_suit: None,
            achieved_points: 0,
        };
        //let safe = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        //let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        let res = solver.ab_tt(local_state, 0, 120);
        //assert_eq!(safe.0, 7);
        assert_eq!(res, 7);
    }

    #[test]
    fn minmax_v2_paper_one_1() {
        let player_one = KREUZ_JACK | KREUZ_TEN
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | HEARTS_JACK |  KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE |  KARO_SEVEN;
        let all_cards = player_one | player_two | player_three;
        assert_eq!(all_cards.0.count_ones(), 18);

        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::Two,
            current_suit: None,
            achieved_points: 0,
        };

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.ab_tt(local_state, 0, 120);

        assert_eq!(result, 7);
    }

    #[test]
    fn minmax_v2_paper_one() {
        let player_one = KREUZ_JACK | KREUZ_TEN
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_KING | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT | KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE | PIQUS_EIGHT| KARO_SEVEN;
        let all_cards = player_one | player_two | player_three;
        assert_eq!(all_cards.0.count_ones(), 21);
        assert_eq!(player_one & player_two & player_three, BitCards(0));
        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.ab_tt(local_state, 0, 120);
        assert_eq!(result, 7);
    }

    #[test]
    #[ignore]
    fn minmax_v2_paper_two() {
        let player_1 =
            BitCards(
                KREUZ_SEVEN.0 | KREUZ_EIGHT.0 | KREUZ_QUEEN.0 | KREUZ_KING.0 | KREUZ_ASS.0 |
                    KARO_SEVEN.0 | KARO_TEN.0 | KARO_ASS.0 | PIQUS_ASS.0);
        let player_2 =
            BitCards(
                HEARTS_JACK.0 | KARO_EIGHT.0 | KARO_NINE.0 | KARO_QUEEN.0 | PIQUS_NINE.0 |
                    HEARTS_SEVEN.0 | HEARTS_EIGHT.0 | HEARTS_NINE.0 | HEARTS_QUEEN.0);
        let player_3 =
            BitCards(
                KARO_JACK.0 | KREUZ_NINE.0 | HEARTS_ASS.0 | PIQUS_SEVEN.0 | PIQUS_EIGHT.0 |
                    PIQUS_QUEEN.0 | PIQUS_KING.0 | PIQUS_TEN.0 | KARO_KING.0);

        let all_cards = player_1 | player_2 | player_3;
        assert_eq!(all_cards.0.count_ones(), 27);
        assert_eq!(all_cards.get_cards_points(), 92);

        let global_state = GlobalState::new(
            (player_1, player_2, player_3),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };
        //let result = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        assert_eq!(result.0, 78)
    }



    #[test]
    fn minmax_v2_very_small() {
        let player1 = BitCards(KREUZ_JACK.0);
        let player2 = BitCards(HEARTS_EIGHT.0);
        let player3 = BitCards(HEARTS_QUEEN.0);

        let all = player1 | player2 | player3;
        let global_state = GlobalState::new(
            (player1, player2, player3),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );
        let local_state = LState {
            remaining_cards: all,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };
        //let result_normal = minimax(local_state, &global_state);
        //let result = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        //assert_eq!(result_normal.0, 5);
        //assert_eq!(result.0, result_normal.0);

        let player1 = KREUZ_JACK | KREUZ_TEN;
        let player2 = HEARTS_JACK | PIQUS_JACK;
        let player3 = KREUZ_QUEEN | KREUZ_SEVEN;

        let global_state = GlobalState::new(
            (player1, player2, player3),
            BitCards(0),
            Player::One,
            Variant::Clubs,
        );

        let local_state = LState {
            remaining_cards: player1 | player2 | player3,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            achieved_points: 0,
        };

        let mut solver = Solver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);

        //let result = minimax_v2(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 4);
    }
}