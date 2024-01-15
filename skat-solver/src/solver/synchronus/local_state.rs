use std::f32::consts::E;
use arrayvec::ArrayVec;
use crate::solver::bitboard::{BitCard, BitCards, EMPTY_CARD};
use crate::solver::{calculate_current_suit_mask, calculate_next_moves, calculate_who_won, calculate_winner, GlobalState, Player};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LState {
    pub remaining_cards: BitCards,
    // 30 bit
    pub current_played_cards: (BitCard, BitCard),
    // 0 bit
    pub current_player: Player,
    //2 bits
    pub current_suit: Option<BitCards>,
    // does not need to be stored
    pub achieved_points: u8, // 0 bit
}

impl LState {
    
    pub(crate) fn new(remaining_cards: BitCards, current_player: Player) -> LState {
        LState {
            remaining_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player,
            current_suit: None,
            achieved_points: 0
        }
    }
    
    pub(crate) fn get_hash(&self) -> (u32, Player) {
        (self.remaining_cards.0, self.current_player)
    }

    pub(crate) fn get_hash_better(&self, global_state: &GlobalState) -> u32 {
        match self.current_player {
            Player::One => {
                self.remaining_cards.0
            }
            Player::Two => {
                (self.remaining_cards | global_state.skat).0
            }
            Player::Three => {
                (self.remaining_cards | global_state.skat.get_next_card_in_binary()).0
            }
        }
    }
}



impl LState {
    pub(crate) fn is_terminal(&self) -> bool {
        self.remaining_cards.0 == 0
    }

    #[inline(always)]
    pub(crate) fn is_max_node(&self, global_state: &GlobalState) -> bool {
        self.current_player == global_state.alone_player
    }

    #[inline(always)]
    fn get_available(&self, current_player: Player, global_state: &GlobalState) -> BitCards {
        match current_player {
            Player::One => global_state.player_cards.0 & self.remaining_cards,
            Player::Two => global_state.player_cards.1 & self.remaining_cards,
            Player::Three => global_state.player_cards.2 & self.remaining_cards,
        }
    }

    fn get_full_next_state(&self, global_state: &GlobalState) -> ArrayVec<LState, 1000> {
        if self.is_terminal() {
            return ArrayVec::new();
        }
        let mut next_states: ArrayVec<LState, 1000> = ArrayVec::new();
        let current_player = self.get_available(self.current_player, global_state);
        let next_player = self.get_available(self.current_player.get_next_player(), global_state);
        let next_next_player = self.get_available(self.current_player.get_next_player().get_next_player(), global_state);

        for first_move in current_player {
            let suit = Some(calculate_current_suit_mask(first_move, &global_state.variant));
            let available_player_two = calculate_next_moves(next_player, suit);
            for second_move in available_player_two {
                let available_player_three = calculate_next_moves(next_next_player, suit);
                for third_move in available_player_three {
                    let remaining_cards = BitCards(self.remaining_cards.0 & (!(first_move | second_move | third_move).0));
                    let winner_card = calculate_who_won((first_move, second_move), third_move, &global_state.variant);
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
                    )
                }
            }

        }
        next_states
    }

    pub(crate) fn get_next_states(&self, global_state: &GlobalState) -> ArrayVec<LState, 10> {
        let mut next_states: ArrayVec<LState, 10> = ArrayVec::new();
        //get available cards
        let available = self.get_available(self.current_player, global_state);
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






