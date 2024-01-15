use std::cmp::{max, min};
use arrayvec::ArrayVec;
use crate::solver::bitboard::{BitCard, BitCards,EMPTY_CARD};
use crate::solver::{calculate_current_suit_mask, calculate_next_moves, calculate_who_won, calculate_winner, GlobalState, Player};

#[derive(Debug, Clone, Copy, PartialEq)]
struct LocalState {
    remaining_cards: BitCards,
    // 32 bit
    current_played_cards: (BitCard, BitCard),
    // 5 + 5 bit
    current_player: Player,
    //2 bits
    current_suit: Option<BitCards>,
    // does not need to be stored
    current_points_alone: u8, // 7 bit 0-120 < 128
    // total = 51 bit
}

impl LocalState {
    fn new(remaining_cards: BitCards) -> LocalState {
        LocalState {
            remaining_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 0
        }
    }

    fn is_terminal(&self) -> bool {
        self.remaining_cards.0 == 0
    }

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

    fn get_next_state(&self, next_move: BitCard, global_state: &GlobalState) -> LocalState {
        let available = self.get_available(global_state);
        assert_ne!(available.0 & next_move.0, 0);
        let next_player = self.current_player.get_next_player();
        let remaining_cards = BitCards(self.remaining_cards.0 & (!next_move.0));
        match self.current_played_cards {
            (BitCard(0), BitCard(0)) => {
                LocalState {
                    remaining_cards,
                    current_played_cards: (next_move, EMPTY_CARD),
                    current_player: next_player,
                    current_suit: Some(calculate_current_suit_mask(next_move, &global_state.variant)),
                    current_points_alone: self.current_points_alone,
                }
            }
            (_, BitCard(0)) => {
                LocalState {
                    remaining_cards,
                    current_played_cards: (self.current_played_cards.0, next_move),
                    current_player: next_player,
                    current_suit: self.current_suit,
                    current_points_alone: self.current_points_alone,
                }
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
                LocalState {
                    remaining_cards,
                    current_played_cards: (BitCard(0), BitCard(0)),
                    current_player: winner_player,
                    current_suit: None,
                    current_points_alone: self.current_points_alone + winner_points,
                }
            }
        }
    }


    fn get_next_states(&self, global_state: &GlobalState) -> ArrayVec<LocalState, 10> {
        let mut next_states: ArrayVec<LocalState, 10> = ArrayVec::new();
        //get available cards
        let available = self.get_available(global_state);
        let possible_moves: BitCards = calculate_next_moves(available, self.current_suit);
        let next_player = self.current_player.get_next_player();
        for next_move in possible_moves {
            let remaining_cards = BitCards(self.remaining_cards.0 & (!next_move.0));
            match self.current_played_cards {
                (BitCard(0), BitCard(0)) => {
                    next_states.push(
                        LocalState {
                            remaining_cards,
                            current_played_cards: (next_move, BitCard(0)),
                            current_player: next_player,
                            current_suit: Some(calculate_current_suit_mask(next_move, &global_state.variant)),
                            current_points_alone: self.current_points_alone,
                        }
                    );
                }
                (_, BitCard(0)) => {
                    next_states.push(
                        LocalState {
                            remaining_cards,
                            current_played_cards: (self.current_played_cards.0, next_move),
                            current_player: next_player,
                            current_suit: self.current_suit,
                            current_points_alone: self.current_points_alone,
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
                        LocalState {
                            remaining_cards,
                            current_played_cards: (BitCard(0), BitCard(0)),
                            current_player: winner_player,
                            current_suit: None,
                            current_points_alone: self.current_points_alone + winner_points,
                        }
                    );
                }
            };
        }
        next_states
    }
}




fn minimax(local_state: LocalState, global_state: &GlobalState, alpha: u8, beta: u8) -> (u8, Option<LocalState>) {
    if local_state.is_terminal() {
        return (local_state.current_points_alone + global_state.skat_points, None);
    }
    let mut result: u8;
    let mut optimal_move: Option<LocalState> = None;
    let mut alpha = alpha;
    let mut beta = beta;
    if local_state.is_max_node(global_state) {
        result = 0;
        for next_state in local_state.get_next_states(global_state) {
            let (next_result, _) = minimax(next_state, global_state, alpha, beta);
            if next_result > result {
                result = next_result;
                optimal_move = Some(next_state);
            }
            alpha = max(alpha, result);
            if beta <= alpha {
                break; // Beta cut-off
            }
        }
    } else {
        result = 120;
        for next_state in local_state.get_next_states(global_state) {
            let (next_result, _) = minimax(next_state, global_state, alpha, beta);
            if next_result < result {
                result = next_result;
                optimal_move = Some(next_state);
            }
            beta = min(beta, result);
            if beta <= alpha {
                break; // Alpha cut-off
            }
        }
    }
    (result, optimal_move)
}



#[cfg(test)]
mod tests {
    use crate::solver::bitboard::{BitCards, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN};
    use crate::solver::{GlobalState, Player};
    use crate::solver::synchronus::alpha_beta::{LocalState, minimax};
    use crate::solver::Variant::Clubs;


    #[test]
    fn alpha_beta_paper_one() {
        let player_one = KREUZ_JACK | KREUZ_TEN
            | HEARTS_TEN | HEARTS_KING | HEARTS_EIGHT | PIQUS_KING | PIQUS_SEVEN;
        let player_two = PIQUS_JACK | HEARTS_JACK | KREUZ_EIGHT | KARO_ASS | KARO_TEN | KARO_QUEEN | KARO_NINE;
        let player_three = KREUZ_QUEEN | KREUZ_SEVEN | HEARTS_ASS | HEARTS_SEVEN | PIQUS_NINE | PIQUS_EIGHT| KARO_SEVEN;
        let all_cards = player_one | player_two | player_three;

        let local_state = LocalState::new(all_cards);
        let global_state = GlobalState::new((player_one, player_two, player_three), BitCards(0), Player::One, Clubs);
        //let result_minmax = minimax(local_state, &global_state);
        let result_alpha_beta = minimax(local_state, &global_state, 0, 120);
        //assert_eq!(result_minmax.0, 7);
        assert_eq!(result_alpha_beta.0, 7)
    }


    #[test]
    fn alpha_beta_paper_two() {
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
            Clubs,
        );
        let local_state = LocalState::new(all_cards);
        let result = minimax(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 78)
    }


}