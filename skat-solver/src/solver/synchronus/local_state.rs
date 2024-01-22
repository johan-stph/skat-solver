use arrayvec::ArrayVec;
use crate::solver::bitboard::{BitCard, BitCards, calculate_who_won_better, EMPTY_CARD};
use crate::solver::{calculate_current_suit_mask, calculate_next_moves, calculate_winner, GlobalState, Player};

pub struct LStateAdvanced {
    pos: u64 // (30 bit remaining_cards + 2 bit current_player) + 32 bit current_suit
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LState {
    pub remaining_cards: BitCards, // 30 bit
    pub current_player: Player, //2 bit
    // 30 bit
    pub current_played_cards: (BitCard, BitCard), //could be stored in 32 bit or 0 if i and with all_cards
    //2 bits
    pub current_suit: Option<BitCards>, // 0-5 -> 32 bit or 3bit
}

impl LState {

    pub fn new(remaining_cards: BitCards, current_player: Player) -> LState {
        LState {
            remaining_cards,
            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
            current_player,
            current_suit: None,
        }
    }

    pub(crate) fn get_hash(&self) -> (u32, Player) {
        (self.remaining_cards.0, self.current_player)
    }

    #[inline(always)]
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

    #[inline(always)]
    pub(crate) fn is_full_node(&self) -> bool {
        self.current_suit.is_none()
    }
}



impl LState {

    #[inline(always)]
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

    #[inline(always)]
    pub(crate) fn get_next_states(&self, global_state: &GlobalState) -> ArrayVec<(LState, u8, u8), 10>{
        let mut move_sorter = MoveSorter::new();
        //get available cards
        let available = self.get_available(self.current_player, global_state);
        let possible_moves: BitCards = calculate_next_moves(available, self.current_suit);
        let next_player = self.current_player.get_next_player();
        for next_move in possible_moves {
            let remaining_cards = BitCards(self.remaining_cards.0 & (!next_move.0));
            match self.current_played_cards {
                (BitCard(0), BitCard(0)) => unsafe {
                    move_sorter.add(
                        LState {
                            remaining_cards,
                            current_played_cards: (next_move, EMPTY_CARD),
                            current_player: next_player,
                            current_suit: Some(calculate_current_suit_mask(next_move, &global_state.variant)),
                        }, next_move.get_point(),0)
                }
                (_, BitCard(0)) => unsafe {
                    move_sorter.add(
                        LState {
                            remaining_cards,
                            current_played_cards: (self.current_played_cards.0, next_move),
                            current_player: next_player,
                            current_suit: self.current_suit,
                        }, self.current_played_cards.0.get_point() + next_move.get_point() , 0);
                }
                (_, _) => unsafe {
                    let winner_card = calculate_who_won_better(self.current_played_cards.0, self.current_played_cards.1, next_move, &global_state.variant);
                    //if winner_card is alone_player add points
                    let winner_player = calculate_winner(winner_card.0, global_state);
                    let winner_points = if winner_player == global_state.alone_player {
                        winner_card.1
                    } else {
                        0
                    };
                    move_sorter.add(
                        LState {
                            remaining_cards,
                            current_played_cards: (EMPTY_CARD, EMPTY_CARD),
                            current_player: winner_player,
                            current_suit: None,
                        }, winner_points, winner_points
                    );
                }
            };
        }
        move_sorter.sort();
        move_sorter.entries
    }

}

pub struct MoveSorter {
    //STATE, expected, real (or 0)
    entries: ArrayVec<(LState, u8, u8), 10>,
}

impl MoveSorter {
    pub fn new() -> MoveSorter {
        MoveSorter {
            entries: ArrayVec::new(),
        }
    }

    pub unsafe fn add(&mut self, move_: LState, score: u8, real: u8) {
        self.entries.push_unchecked((move_, score, real))
    }
    pub fn sort(&mut self) {
        self.entries.sort_by(
            |a , b| {
                //can be left out and just do the normal comparison
                if a.0.is_full_node() {
                    return b.2.cmp(&a.2)
                }
                b.1.cmp(&a.1)
            }
        )
    }
}






