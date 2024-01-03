use std::cmp::{max, min};
use arrayvec::ArrayVec;
use crate::bitboard::{BitCard, BitCards, EMPTY_CARD, Variant};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Player {
    One,
    Two,
    Three,
}

impl Player {
    fn get_next_player(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::Three,
            Player::Three => Player::One,
        }
    }
}

struct GlobalState {
    player_cards: (BitCards, BitCards, BitCards),
    skat: BitCards,
    alone_player: Player,
    variant: Variant,
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct LocalState {
    remaining_cards: BitCards, // 32 bit
    current_played_cards: (BitCard, BitCard), // 5 + 5 bit
    current_player: Player, //2 bits
    current_suit: Option<BitCards>, // does not need to be stored
    current_points_alone: u8, // 7 bit 0-120 < 128
    // total = 51 bit
}

impl LocalState {
    fn get_hash(&self) -> u32 {

        todo!("implement")
    }
}

impl LocalState {
    fn is_terminal(&self) -> bool {
        self.remaining_cards.0 == 0
    }

    fn get_points(&self, global_state: &GlobalState) -> u8 {
        self.current_points_alone + global_state.get_skat_points()
    }
    fn is_max_node(&self, global_state: &GlobalState) -> bool {
        self.current_player == global_state.alone_player
    }
    fn get_next_state(&self, next_move: BitCard, global_state: &GlobalState) -> LocalState {
        let available: BitCards = match self.current_player {
            Player::One => global_state.player_cards.0 & self.remaining_cards,
            Player::Two => global_state.player_cards.1 & self.remaining_cards,
            Player::Three => global_state.player_cards.2 & self.remaining_cards,
        };
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
                let winner_card = calculate_who_won(self.current_played_cards, &next_move, &global_state.variant);
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
        let available: BitCards = match self.current_player {
            Player::One => global_state.player_cards.0 & self.remaining_cards,
            Player::Two => global_state.player_cards.1 & self.remaining_cards,
            Player::Three => global_state.player_cards.2 & self.remaining_cards,
        };
        let possible_moves: BitCards = calculate_next_moves(&available, self.current_suit);
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
                    let winner_card = calculate_who_won(self.current_played_cards, &next_move, &global_state.variant);
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



fn calculate_current_suit_mask(first_card: BitCard, variant: &Variant) -> BitCards {
    if first_card.0 & variant.get_binary_mask() != 0 {
        return BitCards(variant.get_binary_mask());
    }
    BitCards(first_card.get_color_mask())
}

fn calculate_winner(winning_card: BitCard, global_state: &GlobalState) -> Player {
    if winning_card.0 & global_state.player_cards.0.0 != 0 {
        Player::One
    } else if winning_card.0 & global_state.player_cards.1.0 != 0 {
        Player::Two
    } else {
        Player::Three
    }
}

fn calculate_who_won(current_played_cards: (BitCard, BitCard), last_card: &BitCard, variant: &Variant) -> (BitCard, u8) {
    let winning_card = if current_played_cards.0.greater_than(&current_played_cards.1, variant) {
        if current_played_cards.0.greater_than(last_card, variant) {
            current_played_cards.0
        } else {
            *last_card
        }
    } else if current_played_cards.1.greater_than(last_card, variant) {
        current_played_cards.1
    } else {
        *last_card
    };
    (winning_card, current_played_cards.0.get_point() + current_played_cards.1.get_point() + last_card.get_point())
}


fn calculate_next_moves(current_cards: &BitCards, suit_mask: Option<BitCards>) -> BitCards {
    if suit_mask.is_none() {
        return *current_cards;
    }
    //now the option must exist
    let suit_mask = suit_mask.unwrap();
    let available = suit_mask & *current_cards;
    if available.0 != 0 {
        return available;
    }
    *current_cards
}


fn minimax(local_state: LocalState, global_state: &GlobalState, alpha: u8, beta: u8) -> (u8, Option<LocalState>) {
    if local_state.is_terminal() {
        return (local_state.get_points(global_state), None);
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

impl GlobalState {
    fn get_skat_points(&self) -> u8 {
        let mut result = 0;
        for card in self.skat {
            result += card.get_point();
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use crate::bitboard::{BitCard, BitCards, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN, Variant};
    use crate::solver::{calculate_current_suit_mask, GlobalState, LocalState, minimax, Player};

    #[test]
    fn minimax_simple() {
        let global_state = GlobalState {
            player_cards: (BitCards(0b01000100001001011000000011010100), BitCards(0b10010000110000100100001000001011), BitCards(0b00101011000110000001110100000000)),
            skat: BitCards(0b00000000000000000010000000100000),
            alone_player: Player::Three,
            variant: Variant::Grand,
        };
        let local_state = LocalState {
            remaining_cards: BitCards(u32::MAX),
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 0,
        };
        //let result = minimax(local_state, &global_state);
        //dbg!(result);
    }

    #[test]
    #[ignore]
    fn minimax_paper() {
        let global_state = GlobalState {
            player_cards: (BitCards(0b10000100000001000101100100000000), BitCards(0b01100000010000000000000001101100), BitCards(0b00000001001000011010000010000001)),
            skat: BitCards(0),
            alone_player: Player::One,
            variant: Variant::Clubs,
        };


        let local_state = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0),
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 0,
        };
        let res_1 = minimax(local_state, &global_state, 0, 120);
        let local_state_2 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0) & BitCards(!PIQUS_KING.0),
            current_played_cards: (PIQUS_KING, BitCard(0)),
            current_player: Player::Two,
            current_suit: Some(calculate_current_suit_mask(PIQUS_KING, &Variant::Clubs)),
            current_points_alone: 0,
        };
        assert_eq!(res_1.1.unwrap(), local_state_2);
        let res_2 = minimax(local_state_2, &global_state, 0, 120);
        let local_state_3 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0) & BitCards(!PIQUS_KING.0) & BitCards(!KREUZ_EIGHT.0),
            current_played_cards: (PIQUS_KING, KREUZ_EIGHT),
            current_player: Player::Three,
            current_suit: Some(calculate_current_suit_mask(PIQUS_KING, &Variant::Clubs)),
            current_points_alone: 0,
        };
        assert_eq!(res_2.1.unwrap(), local_state_3);
        let res_3 = minimax(local_state_3, &global_state, 0, 120);
        let local_state_4 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0) & BitCards(!PIQUS_KING.0) & BitCards(!KREUZ_EIGHT.0) & BitCards(!PIQUS_EIGHT.0),
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::Two,
            current_suit: None,
            current_points_alone: 0,
        };
        assert_eq!(res_3.1.unwrap(), local_state_4);
        let res_4 = minimax(local_state_4, &global_state, 0, 120);
        let local_state_5 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0) & BitCards(!PIQUS_KING.0) & BitCards(!KREUZ_EIGHT.0) & BitCards(!PIQUS_EIGHT.0) & BitCards(!HEARTS_JACK.0),
            current_played_cards: (HEARTS_JACK, BitCard(0)),
            current_player: Player::Three,
            current_suit: Some(calculate_current_suit_mask(HEARTS_JACK, &Variant::Clubs)),
            current_points_alone: 0,
        };
        assert_eq!(res_4.1.unwrap(), local_state_5);
        let res_5 = minimax(local_state_5, &global_state, 0, 120);
        let local_state_6 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0)
                & BitCards(!PIQUS_KING.0)
                & BitCards(!KREUZ_EIGHT.0)
                & BitCards(!PIQUS_EIGHT.0)
                & BitCards(!HEARTS_JACK.0)
                & BitCards(!KREUZ_SEVEN.0),
            current_played_cards: (HEARTS_JACK, KREUZ_SEVEN),
            current_player: Player::One,
            current_suit: Some(calculate_current_suit_mask(HEARTS_JACK, &Variant::Clubs)),
            current_points_alone: 0,
        };
        assert_eq!(res_5.1.unwrap(), local_state_6);
        let res_6 = minimax(local_state_6, &global_state, 0, 120);
        let local_state_7 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0)
                & BitCards(!PIQUS_KING.0)
                & BitCards(!KREUZ_EIGHT.0)
                & BitCards(!PIQUS_EIGHT.0)
                & BitCards(!HEARTS_JACK.0)
                & BitCards(!KREUZ_SEVEN.0)
                & BitCards(!KREUZ_TEN.0)
            ,
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::Two,
            current_suit: None,
            current_points_alone: 0,
        };
        assert_eq!(res_6.1.unwrap(), local_state_7);
        let res_7 = minimax(local_state_7, &global_state, 0, 120);
        let local_state_8 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0)
                & BitCards(!PIQUS_KING.0)
                & BitCards(!KREUZ_EIGHT.0)
                & BitCards(!PIQUS_EIGHT.0)
                & BitCards(!HEARTS_JACK.0)
                & BitCards(!KREUZ_SEVEN.0)
                & BitCards(!KREUZ_TEN.0)
                & BitCards(!KARO_NINE.0)
            ,
            current_played_cards: (KARO_NINE, BitCard(0)),
            current_player: Player::Three,
            current_suit: Some(calculate_current_suit_mask(KARO_NINE, &Variant::Clubs)),
            current_points_alone: 0,
        };
        assert_eq!(res_7.1.unwrap(), local_state_8);
        let res_8 = minimax(local_state_8, &global_state, 0, 120);
        let local_state_9 = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0)
                & BitCards(!PIQUS_KING.0)
                & BitCards(!KREUZ_EIGHT.0)
                & BitCards(!PIQUS_EIGHT.0)
                & BitCards(!HEARTS_JACK.0)
                & BitCards(!KREUZ_SEVEN.0)
                & BitCards(!KREUZ_TEN.0)
                & BitCards(!KARO_NINE.0)
                & BitCards(!KARO_SEVEN.0)
            ,
            current_played_cards: (KARO_NINE, KARO_SEVEN),
            current_player: Player::One,
            current_suit: Some(calculate_current_suit_mask(KARO_NINE, &Variant::Clubs)),
            current_points_alone: 0,
        };
        assert_eq!(res_8.1.unwrap(), local_state_9);
        let res_9 = minimax(local_state_9, &global_state, 0, 120);

        let local_state_clemens = LocalState {
            remaining_cards: BitCards(global_state.player_cards.0.0 | global_state.player_cards.1.0 | global_state.player_cards.2.0)
                & BitCards(!PIQUS_KING.0)
                & BitCards(!KARO_NINE.0)
                & BitCards(!PIQUS_EIGHT.0),
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 4,
        };
        let clemens_result = minimax(local_state_clemens, &global_state, 0, 120);
        //dbg!(clemens_result);

        //let result = minimax(local_state_10, &global_state, 0, 120);
        //dbg!(result.1.unwrap().current_played_cards.1.get_human_representation());
        //dbg!(result);
    }

    #[test]
    fn setup() {
        let first = BitCards(0b01000100001001011000000011010100);
        let second = BitCards(0b10010000110000100100001000001011);
        let third = BitCards(0b00101011000110000001110100000000);
        let skat = BitCards(0b00000000000000000010000000100000);
        let all = BitCards(0b11111111111111111111111111111111);
        let new = BitCards(first.0 | second.0 | third.0 | skat.0);
        assert_eq!(first.0 & second.0 & third.0 & skat.0, 0);
        assert_eq!(new.0, all.0);
    }

    #[test]
    fn setup_paper() {
        let resulting = BitCards(0b10000100000001000101100100000000) | BitCards(0b01100000010000000000000001101100) | BitCards(0b00000001001000011010000010000001);
        assert_eq!(resulting.get_cards_points(), 72);
    }

    #[test]
    #[ignore]
    fn setup_game_2() {
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

        let global_state = GlobalState {
            player_cards: (player_1, player_2, player_3),
            skat: BitCards(0),
            alone_player: Player::One,
            variant: Variant::Clubs,
        };
        let local_state = LocalState {
            remaining_cards: all_cards,
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 0,
        };
        let result = minimax(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 78)
    }

    #[test]
    fn long_game() {
        let player_2 =
            BitCards(
                HEARTS_JACK.0 | KREUZ_ASS.0 | KREUZ_TEN.0 | KREUZ_SEVEN.0 | KARO_ASS.0 |
                    KARO_SEVEN.0 | KARO_NINE.0 | PIQUS_ASS.0 | PIQUS_KING.0 | PIQUS_QUEEN.0);
        let player_1 =
            BitCards(
                KARO_JACK.0 | KREUZ_KING.0 | KREUZ_EIGHT.0 | HEARTS_ASS.0 | HEARTS_SEVEN.0 |
                    PIQUS_TEN.0 | PIQUS_SEVEN.0 | KARO_TEN.0 | KARO_QUEEN.0 | KARO_EIGHT.0);
        let player_3 =
            BitCards(
                KREUZ_JACK.0 | PIQUS_JACK.0 | KREUZ_QUEEN.0 | KREUZ_NINE.0 | HEARTS_KING.0 |
                    HEARTS_QUEEN.0 | HEARTS_NINE.0 | HEARTS_EIGHT.0 | PIQUS_NINE.0 | KARO_KING.0);
        let skat = BitCards(
            HEARTS_TEN.0 | PIQUS_EIGHT.0
        );
        let all = player_1 | player_2 | player_3;
        assert_eq!(all.0 | skat.0, u32::MAX);

        let global_state = GlobalState {
            player_cards: (player_1, player_2, player_3),
            skat,
            alone_player: Player::Two,
            variant: Variant::Grand,
        };
        let local_state = LocalState {
            remaining_cards: all,
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 0,
        };
        let result = minimax(local_state, &global_state, 0, 120);
        assert_eq!(result.0, 63)

    }
}