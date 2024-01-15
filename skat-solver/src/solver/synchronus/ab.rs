use crate::solver::GlobalState;
use crate::solver::synchronus::local_state::LState;

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
        if new_alpha >= new_beta {
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
    use crate::solver::bitboard::{BitCards, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN};
    use crate::solver::{GlobalState, Player};
    use crate::solver::synchronus::ab::minimax_with_alpha_beta;
    use crate::solver::synchronus::local_state::LState;
    use crate::solver::Variant::Clubs;

    #[test]
    fn ab_paper_one() {
        let player_one = KREUZ_JACK | KREUZ_TEN
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_KING | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT | KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE | PIQUS_EIGHT| KARO_SEVEN;
        let all_cards = player_one | player_two | player_three;

        let local_state = LState::new(all_cards, Player::One);
        let local_state_copy = local_state;
        let global_state = GlobalState::new((player_one, player_two, player_three), BitCards(0), Player::One, Clubs);
        //let result_minmax = minimax(local_state, &global_state);
        let result_alpha_beta = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        //assert_eq!(result_minmax.0, 7);
        assert_eq!(result_alpha_beta.0, 7)
    }

    #[test]
    fn ab_paper_two() {
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
        let local_state = LState::new(all_cards, Player::One);
        let global_state = GlobalState::new((player_1, player_2, player_3), BitCards(0), Player::One, Clubs);
        let result = minimax_with_alpha_beta(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 78)
    }
}
