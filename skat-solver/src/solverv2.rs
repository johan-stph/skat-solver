use std::cmp::{max, min};
use arrayvec::ArrayVec;
use crate::bitboard::{BitCard, BitCards, calculate_who_won, EMPTY_CARD};
use crate::solver::{calculate_next_moves, calculate_winner, GlobalState, Player};

#[derive(Clone, Copy, Debug)]
pub(crate) struct LState {
    remaining_cards: BitCards,
    // 30 bit
    current_played_cards: (BitCard, BitCard),
    // 0 bit
    current_player: Player,
    //2 bits
    current_suit: Option<BitCards>,
    // does not need to be stored
    achieved_points: u8, // 0 bit
}

impl LState {
    fn get_hash(&self) -> (u32, Player) {
        (self.remaining_cards.0, self.current_player)
    }
}



impl LState {
    fn is_terminal(&self) -> bool {
        self.remaining_cards.0 == 0
    }

    #[inline(always)]
    fn is_max_node(&self, global_state: &GlobalState) -> bool {
        self.current_player == global_state.alone_player
    }

    #[inline(always)]
    fn get_available(&self, global_state: &GlobalState) -> BitCards {
        match self.current_player {
            Player::One => global_state.player_cards.0 & self.remaining_cards,
            Player::Two => global_state.player_cards.1 & self.remaining_cards,
            Player::Three => global_state.player_cards.2 & self.remaining_cards,
        }
    }

    fn get_next_states(&self, global_state: &GlobalState) -> ArrayVec<LState, 10> {
        let mut next_states: ArrayVec<LState, 10> = ArrayVec::new();
        //get available cards
        let available = self.get_available(global_state);
        let possible_moves: BitCards = calculate_next_moves(available, self.current_suit);
        let next_player = self.current_player.get_next_player();
        for next_move in possible_moves {
            let remaining_cards = BitCards(self.remaining_cards.0 & (!next_move.0));
            match self.current_played_cards {
                (BitCard(0), BitCard(0)) => {
                    next_states.push(
                        LState {
                            remaining_cards,
                            current_played_cards: (next_move, BitCard(0)),
                            current_player: next_player,
                            current_suit: Some(calculate_current_suit_mask(next_move, &global_state.variant)),
                            achieved_points: 0,
                        }
                    );
                }
                (_, BitCard(0)) => {
                    next_states.push(
                        LState {
                            remaining_cards,
                            current_played_cards: (self.current_played_cards.0, next_move),
                            current_player: next_player,
                            current_suit: self.current_suit,
                            achieved_points: 0,
                        }
                    )
                }
                (_, _) => {
                    let winner_card = calculate_who_won(self.current_played_cards, next_move, &global_state.variant);
                    //if winner_card is alone_player add points
                    let winner_player = calculate_winner(winner_card.0, global_state);
                    let winner_points = if winner_player == global_state.alone_player {
                        winner_card.1
                    } else {
                        0
                    };
                    next_states.push(
                        LState {
                            remaining_cards,
                            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
                            current_player: winner_player,
                            current_suit: None,
                            achieved_points: winner_points,
                        }
                    );
                }
            };
        }
        next_states
    }
}

fn minimax(local_state: LState, global_state: &GlobalState) -> (u8, Option<LState>) {
    if local_state.is_terminal() {
        return (local_state.achieved_points, None);
    }
    let mut result: u8;
    let mut optimal_move: Option<LState> = None;
    if local_state.is_max_node(global_state) {
        result = 0;
        for next_state in local_state.get_next_states(global_state) {
            let (local_result, _) = minimax(next_state, global_state);
            let next_result = local_state.achieved_points + local_result;
            if next_result > result {
                result = next_result;
                optimal_move = Some(next_state);
            }
        }
    } else {
        result = 120;
        for next_state in local_state.get_next_states(global_state) {
            let (local_result, _) = minimax(next_state, global_state);
            let next_result = local_state.achieved_points + local_result;
            if next_result < result {
                result = next_result;
                optimal_move = Some(next_state);
            }
        }
    }
    (result, optimal_move)
}

fn minimax_v2(local_state: LState, global_state: &GlobalState, mut alpha: i8, mut beta: i8) -> (i8, Option<LState>) {
    if local_state.is_terminal() {
        return (0, None);
    }
    let mut result: i8;
    let mut optimal_move: Option<LState> = None;
    if local_state.is_max_node(global_state) {
        result = 0;
        for next_state in local_state.get_next_states(global_state) {
            let (local_result, _) = minimax_v2(next_state, global_state, alpha, beta);
            let next_result = next_state.achieved_points as i8 + local_result;
            if next_result > result {
                result = next_result;
                optimal_move = Some(next_state);
            }
            alpha = max(alpha, result);
            if beta <= alpha {
                break;
            }
        }
    } else {
        result = 120;
        for next_state in local_state.get_next_states(global_state) {
            let (local_result, _) = minimax_v2(next_state, global_state, alpha, beta);
            let next_result = local_state.achieved_points as i8 + local_result;
            if next_result < result {
                result = next_result;
                optimal_move = Some(next_state);
            }
            beta = min(beta, result);
            if beta <= alpha {
                break;
            }
        }
    }
    (result, optimal_move)
}

fn minimax_with_alpha_beta(local_state: LState, global_state: &GlobalState, alpha: i8, beta: i8) -> (i8, Option<LState>) {
    if local_state.is_terminal() {
        return (0, None);
    }
    let mut optimal_move: Option<LState> = None;
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    for next_state in local_state.get_next_states(global_state) {
        let achieved = next_state.achieved_points as i8;
        let poss_alpha_or_beta = achieved + minimax_with_alpha_beta(next_state,
                                                                    global_state,
                                                                    new_alpha - achieved,
                                                                    new_beta - achieved).0;
        if local_state.is_max_node(global_state) {
            if poss_alpha_or_beta > alpha {
                new_alpha = poss_alpha_or_beta;
                optimal_move = Some(next_state);
            }
        } else if poss_alpha_or_beta < beta {
            new_beta = poss_alpha_or_beta;
            optimal_move = Some(next_state)
        }
        if alpha >= beta {
            break;
        }
    }
    if local_state.is_max_node(global_state) {
        (new_alpha, optimal_move)
    } else {
        (new_beta, optimal_move)
    }
}


#[cfg(test)]
mod tests {
    use crate::bitboard::{BitCard, BitCards, EMPTY_CARD, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN, Variant};
    use crate::solver::{GlobalState, Player};
    use crate::solverv2::{LState, minimax, minimax_v2, minimax_with_alpha_beta};

    #[test]
    #[ignore]
    fn minmax_v2_paper_one() {
        let player_one = KREUZ_JACK | KREUZ_TEN
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_KING | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT | KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE | PIQUS_EIGHT | KARO_SEVEN;
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

        //let result = minimax(local_state, &global_state);
        let result = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 7);
    }

    #[test]
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
        let result = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
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

        let result_normal = minimax(local_state, &global_state);
        let result = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        //let result = minimax_v2(local_state, &global_state, 0, 120);
        assert_eq!(result_normal.0, 4);
        assert_eq!(result.0, 4);
    }
}