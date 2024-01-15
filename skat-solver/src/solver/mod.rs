use crate::solver::bitboard::{BitCard, BitCards, GRAND_MASK, HEARTS_MASK, KARO_MASK};
use crate::solver::Variant::{Clubs, Diamonds, Grand, Hearts, Spades};

mod concurrent;
pub mod synchronus;
pub mod bitboard;


#[derive(Debug, Clone, Copy)]
pub enum Variant {
    Grand,
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl From<u8> for Variant {
    fn from(value: u8) -> Self {
        match value {
            0 => Grand,
            1 => Clubs,
            2 => Spades,
            3 => Hearts,
            4 => Diamonds,
            _ => panic!()
        }
    }
}

impl Variant {
    pub(crate) fn get_binary_mask(&self) -> u32 {
        match self {
            Variant::Grand => {
                GRAND_MASK
            }
            Variant::Diamonds => {
                KARO_MASK | GRAND_MASK
            }
            Variant::Hearts => {
                HEARTS_MASK | GRAND_MASK
            }
            Variant::Spades => {
                bitboard::PIQUS_MASK | GRAND_MASK
            }
            Variant::Clubs => {
                bitboard::KREUZ_MASK | GRAND_MASK
            }
        }
    }
}


#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub enum Player {
    One,
    Two,
    Three,
}

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        match value {
            0 => Player::One,
            1 => Player::Two,
            2 => Player::Three,
            _ => panic!()
        }
    }
}


impl Player {
    pub(crate) fn get_next_player(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::Three,
            Player::Three => Player::One,
        }
    }
}

pub struct GlobalState {
    pub(crate) player_cards: (BitCards, BitCards, BitCards),
    pub(crate) skat: BitCards,
    pub(crate) alone_player: Player,
    pub(crate) variant: Variant,
    pub(crate) skat_points: u8,
}

impl GlobalState {
    pub fn new(player_cards: (BitCards, BitCards, BitCards), skat: BitCards, alone_player: Player, variant: Variant) -> GlobalState {
        GlobalState {
            player_cards,
            skat,
            alone_player,
            variant,
            skat_points: GlobalState::get_skat_points(skat),
        }
    }
    fn get_skat_points(skat: BitCards) -> u8 {
        let mut result = 0;
        for card in skat {
            result += card.get_point();
        }
        result
    }
}

pub(crate) fn calculate_current_suit_mask(first_card: BitCard, variant: &Variant) -> BitCards {
    if first_card.0 & variant.get_binary_mask() != 0 {
        return BitCards(variant.get_binary_mask());
    }
    BitCards(first_card.get_color_mask())
}

pub(crate) fn calculate_winner(winning_card: BitCard, global_state: &GlobalState) -> Player {
    if winning_card.0 & global_state.player_cards.0.0 != 0 {
        Player::One
    } else if winning_card.0 & global_state.player_cards.1.0 != 0 {
        Player::Two
    } else {
        Player::Three
    }
}


pub(crate) fn calculate_next_moves(current_cards: BitCards, suit_mask: Option<BitCards>) -> BitCards {
    match suit_mask {
        None => {
            current_cards
        }
        Some(mask) => {
            let available = mask & current_cards;
            if available.0 != 0 {
                return available;
            }
            current_cards
        }
    }
}


pub(crate) fn calculate_who_won(current_played_cards: (BitCard, BitCard), last_card: BitCard, variant: &Variant) -> (BitCard, u8) {
    let winning_card = if current_played_cards.0.greater_than(current_played_cards.1, variant) {
        if current_played_cards.0.greater_than(last_card, variant) {
            current_played_cards.0
        } else {
            last_card
        }
    } else if current_played_cards.1.greater_than(last_card, variant) {
        current_played_cards.1
    } else {
        last_card
    };
    (winning_card, current_played_cards.0.get_point() + current_played_cards.1.get_point() + last_card.get_point())
}

#[cfg(test)]
mod tests {
    use crate::solver::bitboard::{BitCards, HEARTS_ASS, HEARTS_EIGHT, HEARTS_JACK, HEARTS_KING, HEARTS_NINE, HEARTS_QUEEN, HEARTS_SEVEN, HEARTS_TEN, KARO_ASS, KARO_EIGHT, KARO_JACK, KARO_KING, KARO_NINE, KARO_QUEEN, KARO_SEVEN, KARO_TEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_JACK, KREUZ_KING, KREUZ_NINE, KREUZ_QUEEN, KREUZ_SEVEN, KREUZ_TEN, PIQUS_ASS, PIQUS_EIGHT, PIQUS_JACK, PIQUS_KING, PIQUS_NINE, PIQUS_QUEEN, PIQUS_SEVEN, PIQUS_TEN};
    use crate::solver::{GlobalState, Player, Variant};
    use crate::solver::synchronus::ab_tt::EnhancedSolver;
    use crate::solver::synchronus::local_state::LState;

    #[test]
    fn long_solver_one() {
        let player_one = KARO_JACK | KREUZ_ASS | KREUZ_QUEEN | KREUZ_EIGHT | HEARTS_KING | HEARTS_QUEEN |
            HEARTS_EIGHT | PIQUS_ASS | KARO_ASS | KARO_NINE;
        let player_two = PIQUS_JACK | HEARTS_JACK | HEARTS_NINE | PIQUS_TEN | PIQUS_KING | PIQUS_NINE |
            PIQUS_EIGHT | KARO_TEN | KARO_QUEEN | KARO_EIGHT;
        let player_three = KREUZ_JACK | KREUZ_TEN | KREUZ_KING | KREUZ_NINE | KREUZ_SEVEN | HEARTS_ASS |
            HEARTS_TEN | HEARTS_SEVEN | PIQUS_SEVEN | KARO_SEVEN;
        let skat = PIQUS_QUEEN | KARO_KING;
        let all_cards = player_one | player_two | player_three | skat;
        assert_eq!(all_cards.0, u32::MAX);
        assert_eq!((player_one & player_two).0, 0);
        assert_eq!((player_two & player_three).0, 0);
        assert_eq!((player_three & player_one).0, 0);
        let global_state = GlobalState::new(
            (player_one, player_two, player_three),
            skat,
            Player::One,
            Variant::Grand,
        );
        let local_state = LState::new(player_one | player_two | player_three, Player::One);
        let skatpoints = global_state.skat_points as i8;
        let mut solver = EnhancedSolver {
            global_state,
            look_up_table: Default::default(),
        };
        //let result = solver.minimax_with_alpha_beta_tt(local_state, 0, 120);
        let other_score = solver.ab_tt(local_state, 0, 120);
        assert_eq!(skatpoints + other_score, 29)
    }

    #[test]
    fn long_solver_two() {
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

        let global_state = GlobalState::new(
            (player_1, player_2, player_3),
            skat,
            Player::Two,
            Variant::Grand,
        );
        let local_state = LState::new(all, Player::One);
        //let result = minimax_with_alpha_beta(local_state,&global_state, 0, 120);
        let skat_points = global_state.skat_points;
        let mut solver = EnhancedSolver {
            global_state,
            look_up_table: Default::default(),
        };

        let result = solver.ab_tt(local_state, 0, 120);
        assert_eq!(result + skat_points as i8, 63)
    }
}

