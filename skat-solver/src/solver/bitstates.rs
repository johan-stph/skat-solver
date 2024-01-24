use arrayvec::ArrayVec;
use crate::solver::bitboard::{BitCard, BitCards, calculate_who_won_better, GRAND_MASK, HEARTS_MASK, KARO_MASK, KREUZ_MASK, PIQUS_MASK, SEVEN_OR_EIGHT_OR_NINE};
use crate::solver::{Player, Variant};

///
/// Represents the global state of a gamge
/// alone_player is always player_one -> local / and gloabl state has to be adjusted if that is not the case
pub struct BitGlobal {
    player_one: BitCards,
    player_two: BitCards,
    player_three: BitCards,
    pub skat: BitCards,
    first_card_skat: u32,
    second_card_skat: u32,
    color_masks: [u32; 5],
    //not relevant
    pub skat_points: u8,
    //relevant *5
    variant: Variant
}

const fn generate_color_mask(variant: Variant) -> [u32; 5] {
    match variant {
        Variant::Grand => {
            [GRAND_MASK, KREUZ_MASK, PIQUS_MASK, HEARTS_MASK, KARO_MASK]
        }
        Variant::Clubs => {
            [KREUZ_MASK | GRAND_MASK, PIQUS_MASK, HEARTS_MASK, KARO_MASK, 0]
        }
        Variant::Spades => {
            [KREUZ_MASK, PIQUS_MASK | GRAND_MASK, HEARTS_MASK, KARO_MASK, 0]
        }
        Variant::Hearts => {
            [KREUZ_MASK, PIQUS_MASK, HEARTS_MASK | GRAND_MASK, KARO_MASK, 0]
        }
        Variant::Diamonds => {
            [KREUZ_MASK, PIQUS_MASK, HEARTS_MASK, KARO_MASK | GRAND_MASK, 0]
        }
    }

}

impl BitGlobal {
    pub fn new(pl_one: BitCards, pl_two: BitCards, pl_three: BitCards, variant: Variant) -> BitGlobal {
        let mut skat = BitCards(!(pl_one | pl_two | pl_three).0);
        let first_card = skat.get_next_card_in_binary().0;
        let second_card;
        let skat_points;
        if (pl_one | pl_two | pl_three).0.count_zeros() != 2 {
            //create custom skatl
            second_card = BitCards(skat.0 & (!first_card)).get_next_card_in_binary().0;
            skat = BitCards(first_card | second_card);
            skat_points = 0;
        } else {
            second_card = skat.0 & !first_card;
            skat_points = skat.get_cards_points();
        }
        let color_masks = generate_color_mask(variant);
        BitGlobal {
            player_one: pl_one,
            player_two: pl_two,
            player_three: pl_three,
            skat,
            first_card_skat: first_card,
            second_card_skat: second_card,
            color_masks,
            skat_points,
            variant,
        }

    }
}


impl BitGlobal {
    #[inline(always)]
    fn calculate_winner_state(&self, winner_move: BitCard, points: u8, old_state: u32) -> (u32, u8) {
        let new_state = old_state & (!self.skat.0);
        if winner_move.0 & self.player_one.0 != 0 {
            return (new_state, points)
        }
        if winner_move.0 & self.player_two.0 !=0 {
            return (new_state | self.first_card_skat, 0)
        }
        (new_state | self.skat.0, 0)
    }
}

///
/// current-player is represented as:
///     pl 1. 00
///     pl 2. 01
///     pl 3. 11
#[derive(Copy, Clone, Debug, )]
pub struct BitLocal {
    //current_player, only with knowledge of skat
    //remaining_cards
    state: u32,
    current_played_cards: (BitCard, BitCard),
    current_suit: u32,
}

impl BitLocal {
    pub fn new(all_cards: u32, current_player: Player, skat: BitCards) -> BitLocal {
        let next = skat.get_next_card_in_binary();
        match current_player {
            Player::One => {
                BitLocal {
                    state: all_cards,
                    current_played_cards: (BitCard(0), BitCard(0)),
                    current_suit: 0,
                }
            }
            Player::Two => {
                BitLocal {
                    state: all_cards | next.0,
                    current_played_cards: (BitCard(0), BitCard(0)),
                    current_suit: 0,
                }
            }
            Player::Three => {
                BitLocal {
                    state: all_cards | skat.0,
                    current_played_cards: (BitCard(0), BitCard(0)),
                    current_suit: 0,
                }
            }
        }
    }
}


impl BitLocal {

    pub fn get_remaining_points(&self, global_state: &BitGlobal) -> u8 {
        BitCards(self.state & !global_state.skat.0).get_cards_points()
    }

    #[inline(always)]
    pub fn is_max_node(&self, global_state: &BitGlobal) -> bool {
        self.state & global_state.skat.0 == 0
    }
    #[inline(always)]
    pub fn get_hash(&self) -> u32 {
        self.state
    }
    #[inline(always)]
    pub fn is_full_node(&self) -> bool {
        self.current_suit == 0
    }

    #[inline(always)]
    pub fn is_terminal(&self, skat: BitCards) -> bool {
        let mut cards = self.state & (!skat.0);
        if self.is_full_node() {
            cards &= !SEVEN_OR_EIGHT_OR_NINE;
        }
        cards == 0
    }

    #[inline(always)]
    fn get_next_player_full_state(&self, global_state: &BitGlobal) -> u32 {
        let player = self.state & global_state.skat.0;
        if player == 0 {
            return self.state | global_state.first_card_skat;
        }
        if player & global_state.second_card_skat == 0 {
            return self.state | global_state.second_card_skat;
        }
        self.state & (!global_state.skat.0)
    }

    #[inline(always)]
    fn get_current_cards(&self, global_state: &BitGlobal) -> BitCards {
        let player = self.state & global_state.skat.0;
        if player == 0 {
            return global_state.player_one & BitCards(self.state);
        }
        if player & global_state.second_card_skat == 0 {
            return global_state.player_two & BitCards(self.state);
        }
        global_state.player_three & BitCards(self.state)
    }

    #[inline(always)]
    fn get_all_cards(&self, global_state: &BitGlobal) -> (u32, u32, u32) {
        let current = self.state & global_state.skat.0;
        let player_one_av = global_state.player_one.0 & self.state;
        let player_two_av = global_state.player_two.0 & self.state;
        let player_three_av = global_state.player_three.0 & self.state;
        if current == 0 {
            return (player_one_av, player_two_av, player_three_av);
        }
        if current & global_state.second_card_skat == 0 {
            return (player_two_av, player_three_av, player_one_av);
        }
        (player_three_av, player_one_av, player_two_av)
    }
}

impl BitLocal {

    pub fn get_next_states(&self, global_state: &BitGlobal) -> ArrayVec<(BitLocal, u8), 10> {
        //determine move ordering if no card is present, one card, and two cards
        let mut next_states: ArrayVec<(BitLocal, u8), 10> = ArrayVec::new();
        if self.is_full_node() {
            for (next_move, current_suit) in self.next_full_cards(global_state) {
                unsafe {
                    next_states.push_unchecked(
                        (BitLocal {
                            state: self.get_next_player_full_state(global_state) & !next_move,
                            current_played_cards: (BitCard(next_move), BitCard(0)),
                            current_suit,
                        }, 0)
                    )
                }
            }
            return next_states;
        }
        let possible = self.get_current_cards(global_state);
        let avaiable = if (possible.0 & self.current_suit) == 0 {
            possible
        } else {
            BitCards(possible.0 & self.current_suit)
        };
        if self.current_played_cards.1.0 == 0 {
            for next_move in avaiable {
                unsafe {
                    next_states.push_unchecked(
                        (BitLocal {
                            state: self.get_next_player_full_state(global_state) & (!next_move.0),
                            current_played_cards: (self.current_played_cards.0, next_move),
                            current_suit: self.current_suit,
                        }, 0)
                    );
                }
            }
            return next_states;
        }
        for next_move in avaiable {
            let winner_card = calculate_who_won_better(self.current_played_cards.0, self.current_played_cards.1, next_move, &global_state.variant);
            let new_state =  global_state.calculate_winner_state(winner_card.0, winner_card.1, self.state);
            unsafe {next_states.push_unchecked(
                (
                    BitLocal {
                        state: new_state.0 & (!next_move.0),
                        current_played_cards: (BitCard(0), BitCard(0)),
                        current_suit: 0,
                    }, new_state.1)
            ) }
        }
        next_states
    }

    fn next_full_cards(&self, bit_global: &BitGlobal) -> ArrayVec<(u32, u32), 10> {
        debug_assert!(self.is_full_node());
        //move, mask
        let mut next_moves: ArrayVec<(u32, u32), 10> = ArrayVec::new();
        let all_cards = self.get_all_cards(bit_global);
        let first: u32 = all_cards.0;
        let second: u32 = all_cards.1;
        let third: u32 = all_cards.2;
        let amount_cards = first.count_ones() as u8;
        let mut color_amount: [(u8, u32); 5] = [(u8::MAX, 0); 5];
        for (i, color_mask) in bit_global.color_masks.iter().enumerate() {
            if first & color_mask == 0 {
                continue;
            }
            let mut c: u8 = 1;
            let second_available = second & color_mask;
            let third_available = third & color_mask;
            if second_available != 0 {
                c *= second_available.count_ones() as u8;
            } else {
                c *= amount_cards
            }
            if third_available != 0 {
                c *= third_available.count_ones() as u8;
            } else {
                c *= amount_cards
            }
            color_amount[i] = (c, *color_mask);
        }
        color_amount.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        for (amount, color) in color_amount {
            if amount == u8::MAX { break; }
            let mut avaialble_in_color = first & color;
            while avaialble_in_color != 0 {
                let msb = get_msb_mask(avaialble_in_color);
                unsafe { next_moves.push_unchecked((msb, color)) };
                avaialble_in_color &= !msb;
            }
        }
        next_moves
    }
}

#[inline(always)]
fn get_msb_mask(num: u32) -> u32 {
    debug_assert!(num != 0);
    1 << (31 - num.leading_zeros())
}

