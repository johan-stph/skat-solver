use std::cmp::{max, min};
use std::collections::HashMap;
use crate::solver::{GlobalState, Player};
use crate::solver::synchronus::ab_tt::Bounds::{LowerBound, UpperBound, Valid};
use crate::solver::synchronus::local_state::LState;


#[derive(Debug)]
pub enum Bounds {
    Valid,
    LowerBound,
    UpperBound
}

pub struct DefaultSolver {
    pub global_state: GlobalState,
    pub look_up_table: HashMap<(u32, Player), (i8, Bounds)>
}


impl DefaultSolver {
    pub fn solve(&mut self, local_state: LState) -> i8 {
        self.ab_tt(local_state, 0, 120) + self.global_state.skat_points as i8
    }


    fn try_insert(&mut self, local_state: &LState, score: i8, bound: Bounds) {
        if local_state.is_full_node() {
            self.look_up_table.insert(local_state.get_hash(), (score, bound));
        }
    }
    fn ab_tt(&mut self, local_state: LState, agoof: i8, bgoof: i8) -> i8 {
        if local_state.is_terminal() {
            return 0;
        }
        let mut new_alpha = agoof;
        let mut new_beta = bgoof;

        if local_state.is_full_node() {
            if let Some(result) = self.look_up_table.get(&local_state.get_hash()) {
                match result.1 {
                    Valid => {
                        return result.0;
                    }
                    LowerBound => {
                        new_alpha = max(new_alpha, result.0)
                    }
                    UpperBound => {
                        new_beta = min(new_beta, result.0)
                    }
                }
                if new_alpha >= new_beta {
                    return result.0;
                }
            }
        }
        let mut changed = false;
        for (next_state, _ , achieved_points) in local_state.get_next_states(&self.global_state) {
            let t_q = achieved_points as i8;
            let succ_val = t_q + self.ab_tt(next_state, new_alpha - t_q, new_beta - t_q);
            if local_state.is_max_node(&self.global_state) {
                if succ_val > new_alpha {
                    changed = true;
                    new_alpha = succ_val;
                }
                //result = max(result, succ_val);
                if new_alpha >= new_beta {
                    self.try_insert(&local_state, new_alpha, LowerBound);
                    return new_alpha;
                }
            } else {
                //result = min(result, succ_val);
                if succ_val < new_beta {
                    changed = true;
                    new_beta = succ_val;
                }
                if new_beta <= new_alpha {
                    self.try_insert(&local_state, new_beta, UpperBound);
                    return new_beta;
                }
            }
        }

        let result = if local_state.is_max_node(&self.global_state) {
            new_alpha
        } else {
            new_beta
        };
        if !local_state.is_full_node() {
            return result;
        }
        if changed {
            self.look_up_table.insert(local_state.get_hash(), (result, Valid));
            return result;
        }
        if local_state.is_max_node(&self.global_state) {
            self.look_up_table.insert(local_state.get_hash(), (result, UpperBound));
        } else {
            self.look_up_table.insert(local_state.get_hash(), (result, LowerBound));
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::solver::bitboard::{BitCards, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN};
    use crate::solver::{GlobalState, Player, Variant};
    use crate::solver::synchronus::ab_tt::DefaultSolver;
    use crate::solver::synchronus::local_state::LState;

    #[test]
    fn ab_tt_paper_one() {
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
        let local_state = LState::new(all_cards, Player::One);

        let mut solver = DefaultSolver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.ab_tt(local_state, 0, 120);
        assert_eq!(result, 7);
    }

    #[test]
    fn ab_tt_paper_two() {
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
        let local_state = LState::new(all_cards, Player::One);
        let mut solver = DefaultSolver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.ab_tt(local_state, 0, 120);
        assert_eq!(result, 78)
    }


    fn run_test(line: &str) -> Result<(), (u8, u8)> {
        let data: Vec<&str> = line.split(',').collect();
        let p1 = BitCards(data[0].parse::<u32>().unwrap());
        let p2 = BitCards(data[1].parse::<u32>().unwrap());
        let p3 = BitCards(data[2].parse::<u32>().unwrap());
        let skat = BitCards(data[3].parse::<u32>().unwrap());
        let current_player: Player = Player::from(data[4].parse::<u8>().unwrap());
        let variant: Variant = Variant::from(data[5].parse::<u8>().unwrap());
        let score = data[6].parse::<u8>().unwrap();
        let local_state = LState::new(p1 | p2 | p3, current_player);
        let global_state = GlobalState::new((p1, p2, p3), skat, Player::One, variant);
        let mut solver = DefaultSolver {
            global_state,
            look_up_table: Default::default(),
        };
        let result = solver.ab_tt(local_state, 0, 120);
        assert!((0..=120).contains(&result));
        let result = result as u8;
        if result == score {
            return Ok(());
        }
        Err((result, score))
    }

    #[test]
    fn ab_tt_normal_four_cards() {
        let input = fs::read_to_string("data/four_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    fn ab_tt_normal_seven_cards() {
        let input = fs::read_to_string("data/seven_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    fn ab_tt_normal_five_cards() {
        let input = fs::read_to_string("data/five_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    fn ab_tt_normal_six_cards() {
        let input = fs::read_to_string("data/six_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    fn ab_tt_normal_one_cards() {
        let input = fs::read_to_string("data/one_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    fn ab_tt_normal_two_cards() {
        let input = fs::read_to_string("data/two_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    fn ab_tt_normal_three_cards() {
        let input = fs::read_to_string("data/three_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }
}
