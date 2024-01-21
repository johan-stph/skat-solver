use std::cmp::{max, min};
use fxhash::FxHashMap;
use crate::solver::GlobalState;
use crate::solver::synchronus::ab_tt::Bounds;
use crate::solver::synchronus::ab_tt::Bounds::{LowerBound, UpperBound, Valid};
use crate::solver::synchronus::local_state::LState;

pub struct EnhancedSolver {
    pub global_state: GlobalState,
    pub look_up_table: FxHashMap<u32, (i8, Bounds)>
}


impl EnhancedSolver {

    pub fn new(global_state: GlobalState) -> EnhancedSolver {
        Self {
            global_state,
            look_up_table: Default::default(),
        }
    }

    fn try_insert(&mut self, local_state: &LState, score: i8, bound: Bounds) {
        if local_state.is_full_node() {
            self.look_up_table.insert(local_state.get_hash_better(&self.global_state), (score, bound));
        }
    }
    fn insert(&mut self, pos: u32, score: i8, bound: Bounds) {
        self.look_up_table.insert(pos, (score, bound));
    }
    pub fn solve(&mut self, local_state: LState) -> u8 {
        //did not improve performance maybe for larger n >7
        let mut min: i8 = 0;
        let mut max = local_state.remaining_cards.get_cards_points() as i8;
        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med {
                med = min / 2;
            } else if med >= 0 && max / 2 > med {
                med = max / 2;
            }
            let r = self.ab_tt(local_state, med, med + 1);   // use a null depth window to know if the actual score is greater or smaller than med
            if r <= med {
                max = r;
            } else {
                min = r;
            }
        }
        min as u8 + self.global_state.skat_points
    }

    pub fn ab_tt(&mut self, local_state: LState, agoof: i8, bgoof: i8) -> i8 {
        if local_state.is_terminal() {
            return 0;
        }
        let mut new_alpha = agoof;
        let mut new_beta = bgoof;

        if local_state.is_full_node() {
            if let Some(result) = self.look_up_table.get(&local_state.get_hash_better(&self.global_state)) {
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
        let mut result;
        if local_state.is_max_node(&self.global_state) {
            result = new_alpha;
        } else {
            result = new_beta;
        }

        for (next_state, achieved_points) in local_state.get_next_states(&self.global_state) {
            let t_q = achieved_points as i8;
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
        if !local_state.is_full_node() {
            return result;
        }

        if local_state.is_max_node(&self.global_state) {
            if result != agoof {
                self.insert(local_state.get_hash_better(&self.global_state), result, Valid);
            }
            else {
                self.insert(local_state.get_hash_better(&self.global_state), result, UpperBound);
            }
        } else if result != bgoof {
            self.insert(local_state.get_hash_better(&self.global_state), result, Valid);
        }
        else {
            self.insert(local_state.get_hash_better(&self.global_state), result, LowerBound);
        }
        result
    }
}
