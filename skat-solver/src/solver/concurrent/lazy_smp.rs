use std::cmp::{max, min};
use fxhash::FxHashMap;
use crate::solver::bitstates::{BitGlobal, BitLocal};
use crate::solver::synchronus::ab_tt::Bounds;
use crate::solver::synchronus::ab_tt::Bounds::{LowerBound, UpperBound, Valid};

pub struct LazyMoreEnhancedSolver {
    pub global_state: BitGlobal,
    pub look_up_table: FxHashMap<u32, (i8, Bounds)>,
}

impl LazyMoreEnhancedSolver {
    pub fn new(global_state: BitGlobal) -> LazyMoreEnhancedSolver {
        Self {
            global_state,
            look_up_table: Default::default(),
        }
    }

    #[inline]
    fn try_insert(&mut self, local_state: &BitLocal, score: i8, bound: Bounds) {
        if local_state.is_full_node() {
            self.look_up_table.insert(local_state.get_hash(), (score, bound));
        }
    }
    #[inline(always)]
    fn insert(&mut self, pos: u32, score: i8, bound: Bounds) {
        self.look_up_table.insert(pos, (score, bound));
    }
    pub fn solve(&mut self, local_state: BitLocal) -> u8 {
        self.ab_tt(local_state, 0, 120) as u8 + self.global_state.skat_points
    }

    pub fn ab_tt(&mut self, local_state: BitLocal, agoof: i8, bgoof: i8) -> i8 {
        if local_state.is_terminal(self.global_state.skat) {
            return 0;
        }
        let mut new_alpha = agoof;
        let mut new_beta = bgoof;
        if local_state.is_full_node() {
            if let Some(result) = self.look_up_table.get(&local_state.get_hash()) {
                match result.1 {
                    Valid => {
                        return result.0
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


        if local_state.is_max_node(&self.global_state) {
            for (next_state, achieved_points) in local_state.get_next_states(&self.global_state) {
                let t_q = achieved_points as i8;
                let succ_val = t_q + self.ab_tt(next_state, new_alpha - t_q, new_beta - t_q);
                new_alpha = max(new_alpha, succ_val);
                if new_alpha >= new_beta {
                    self.try_insert(&local_state, new_alpha, LowerBound);
                    return new_alpha;
                }
            }
            if !local_state.is_full_node() {
                return new_alpha;
            }
            if new_alpha != agoof {
                self.insert(local_state.get_hash(), new_alpha, Valid);
            } else {
                self.insert(local_state.get_hash(), new_alpha, UpperBound);
            }

            return new_alpha
        }
        for (next_state, achieved_points) in local_state.get_next_states(&self.global_state) {
            let t_q = achieved_points as i8;
            let succ_val = t_q + self.ab_tt(next_state, new_alpha - t_q, new_beta - t_q);
            new_beta = min(new_beta, succ_val);
            if new_beta <= new_alpha {
                self.try_insert(&local_state, new_beta, UpperBound);
                return new_beta;
            }
        }
        if !local_state.is_full_node() {
            return new_beta;
        }
        if new_beta != bgoof {
            self.insert(local_state.get_hash(), new_beta, Valid);
        } else {
            self.insert(local_state.get_hash(), new_beta, LowerBound);
        }
        new_beta
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::solver::bitboard::{BitCards};
    use crate::solver::{Player, Variant};
    use crate::solver::bitstates::{BitGlobal, BitLocal};
    use crate::solver::concurrent::lazy_smp::LazyMoreEnhancedSolver;


    fn run_test(line: &str) -> Result<(), (u8, u8)> {
        let data: Vec<&str> = line.split(',').collect();
        let p1 = BitCards(data[0].parse::<u32>().unwrap());
        let p2 = BitCards(data[1].parse::<u32>().unwrap());
        let p3 = BitCards(data[2].parse::<u32>().unwrap());
        let current_player: Player = Player::from(data[4].parse::<u8>().unwrap());
        let variant: Variant = Variant::from(data[5].parse::<u8>().unwrap());
        let score = data[6].parse::<u8>().unwrap();
        let global_state = BitGlobal::new(p1, p2, p3, variant);
        let local_state = BitLocal::new((p1 | p2 | p3).0, current_player, global_state.skat);
        let mut solver = LazyMoreEnhancedSolver::new(global_state);
        let result = solver.solve(local_state);
        assert!((0..=120).contains(&result));
        if result == score {
            return Ok(());
        }
        Err((result, score))
    }

    #[test]
    #[ignore]
    fn lazy_opt2_full() {
        let input = fs::read_to_string("data/full_game.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }

    #[test]
    #[ignore]
    fn lazy_opt2_seven() {
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
    fn lazy_opt2_six() {
        let input = fs::read_to_string("data/six_cards.txt").unwrap();
        let len = input.lines().count();
        let mut successes = 0;

        for line in input.lines() {
            let result = run_test(line);
            if let Ok(()) = result { successes += 1 }
        }
        assert_eq!(successes, len);
    }
}