use crate::solver::GlobalState;
use crate::solver::synchronus::local_state::LState;

pub fn ab(local_state: LState, global_state: &GlobalState, alpha: i8, beta: i8) -> (i8, Option<LState>) {
    if local_state.is_terminal() {
        return (0, None);
    }
    let mut optimal_move: Option<LState> = None;
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    for (next_state, achieved_points) in local_state.get_next_states(global_state) {
        let achieved = achieved_points as i8;
        let poss_alpha_or_beta = achieved + ab(next_state,
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
    use std::fs;
    use crate::solver::bitboard::{BitCards, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN};
    use crate::solver::{GlobalState, Player, Variant};
    use crate::solver::synchronus::ab::{ab};
    use crate::solver::synchronus::local_state::LState;
    use crate::solver::Variant::Clubs;

    #[test]
    fn ab_normal_paper_one() {
        let player_one = KREUZ_JACK | KREUZ_TEN
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_KING | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT | KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE | PIQUS_EIGHT| KARO_SEVEN;
        let all_cards = player_one | player_two | player_three;

        let local_state = LState::new(all_cards, Player::One);
        let global_state = GlobalState::new((player_one, player_two, player_three), BitCards(0), Player::One, Clubs);
        let result_alpha_beta = ab(local_state, &global_state, 0, 120);
        assert_eq!(result_alpha_beta.0, 7)
    }

    #[test]
    fn ab_normal_paper_two() {
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
        let result = ab(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 78)
    }

    fn run_test(line: &str) -> (u8, u8) {
        let data: Vec<&str> = line.split(',').collect();
        let p1 = BitCards(data[0].parse::<u32>().unwrap());
        let p2 = BitCards(data[1].parse::<u32>().unwrap());
        let p3 = BitCards(data[2].parse::<u32>().unwrap());
        let skat= BitCards(data[3].parse::<u32>().unwrap());
        let current_player: Player = Player::from(data[4].parse::<u8>().unwrap());
        let variant: Variant = Variant::from(data[5].parse::<u8>().unwrap());
        let score = data[6].parse::<u8>().unwrap();
        let local_state = LState::new(p1 | p2 | p3, current_player);
        let global_state = GlobalState::new((p1, p2, p3), skat, Player::One, variant);
        let result = ab(local_state, &global_state, 0, 120);
        (result.0 as u8, score)
    }


    #[test]
    fn ab_normal_four_cards() {
        let input = fs::read_to_string("data/four_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
    #[test]
    fn ab_normal_seven_cards() {
        let input = fs::read_to_string("data/seven_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
    #[test]
    fn ab_normal_five_cards() {
        let input = fs::read_to_string("data/five_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
    #[test]
    fn ab_normal_six_cards() {
        let input = fs::read_to_string("data/six_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
    #[test]
    fn ab_normal_one_cards() {
        let input = fs::read_to_string("data/one_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
    #[test]
    fn ab_normal_two_cards() {
        let input = fs::read_to_string("data/two_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
    #[test]
    fn ab_normal_three_cards() {
        let input = fs::read_to_string("data/three_cards.txt").unwrap();
        for line in input.lines() {
            let result = run_test(line);
            assert_eq!(result.0, result.1)
        }
    }
}

