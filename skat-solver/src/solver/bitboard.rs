use std::cmp::max;
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr};
use crate::solver::Variant;

pub(crate) const GRAND_MASK: u32 = !((1 << 28) - 1);
pub(crate) const KREUZ_MASK: u32 = get_binary_mask_for_colors(21, 28);
pub(crate) const PIQUS_MASK: u32 = get_binary_mask_for_colors(14, 21);
pub(crate) const HEARTS_MASK: u32 = get_binary_mask_for_colors(7, 14);
pub(crate) const KARO_MASK: u32 = get_binary_mask_for_colors(0, 7);

const KREUZ_TRUMPF_MASK: u32 = KREUZ_MASK | GRAND_MASK;
const PIQUS_TRUMPF_MASK: u32 = PIQUS_MASK | GRAND_MASK;
const HEARTS_TRUMPF_MASK: u32 = HEARTS_MASK | GRAND_MASK;
const KARO_TRUMPF_MASK: u32 = KARO_MASK | GRAND_MASK;

const SEVEN_MASK: u32 = get_binary_mask_for_rank(1);
const EIGHT_MASK: u32 = get_binary_mask_for_rank(2);
const NINE_MASK: u32 = get_binary_mask_for_rank(3);

const SEVEN_OR_EIGHT_OR_NINE: u32 = SEVEN_MASK | EIGHT_MASK | NINE_MASK;
const QUEEN_MASK: u32 = get_binary_mask_for_rank(4);
const KING_MASK: u32 = get_binary_mask_for_rank(5);
const TEN_MASK: u32 = get_binary_mask_for_rank(6);
const ACE_MASK: u32 = get_binary_mask_for_rank(7);

pub(crate) const EMPTY_CARD: BitCard = BitCard(0);
pub(crate) const KREUZ_JACK: BitCard = BitCard(2_u32.pow(31));
pub(crate) const PIQUS_JACK: BitCard = BitCard(2_u32.pow(30));
pub(crate) const HEARTS_JACK: BitCard = BitCard(2_u32.pow(29));
pub(crate) const KARO_JACK: BitCard = BitCard(2_u32.pow(28));
pub(crate) const KREUZ_ASS: BitCard = BitCard(2_u32.pow(27));
pub(crate) const KREUZ_TEN: BitCard = BitCard(2_u32.pow(26));
pub(crate) const KREUZ_KING: BitCard = BitCard(2_u32.pow(25));
pub(crate) const KREUZ_QUEEN: BitCard = BitCard(2_u32.pow(24));
pub(crate) const KREUZ_NINE: BitCard = BitCard(2_u32.pow(23));
pub(crate) const KREUZ_EIGHT: BitCard = BitCard(2_u32.pow(22));
pub(crate) const KREUZ_SEVEN: BitCard = BitCard(2_u32.pow(21));

pub(crate) const PIQUS_ASS: BitCard = BitCard(2_u32.pow(20));
pub(crate) const PIQUS_TEN: BitCard = BitCard(2_u32.pow(19));
pub(crate) const PIQUS_KING: BitCard = BitCard(2_u32.pow(18));
pub(crate) const PIQUS_QUEEN: BitCard = BitCard(2_u32.pow(17));
pub(crate) const PIQUS_NINE: BitCard = BitCard(2_u32.pow(16));
pub(crate) const PIQUS_EIGHT: BitCard = BitCard(2_u32.pow(15));
pub(crate) const PIQUS_SEVEN: BitCard = BitCard(2_u32.pow(14));

pub(crate) const HEARTS_ASS: BitCard = BitCard(2_u32.pow(13));
pub(crate) const HEARTS_TEN: BitCard = BitCard(2_u32.pow(12));
pub(crate) const HEARTS_KING: BitCard = BitCard(2_u32.pow(11));
pub(crate) const HEARTS_QUEEN: BitCard = BitCard(2_u32.pow(10));
pub(crate) const HEARTS_NINE: BitCard = BitCard(2_u32.pow(9));
pub(crate) const HEARTS_EIGHT: BitCard = BitCard(2_u32.pow(8));
pub(crate) const HEARTS_SEVEN: BitCard = BitCard(2_u32.pow(7));

pub(crate) const KARO_ASS: BitCard = BitCard(2_u32.pow(6));
pub(crate) const KARO_TEN: BitCard = BitCard(2_u32.pow(5));
pub(crate) const KARO_KING: BitCard = BitCard(2_u32.pow(4));
pub(crate) const KARO_QUEEN: BitCard = BitCard(2_u32.pow(3));
pub(crate) const KARO_NINE: BitCard = BitCard(2_u32.pow(2));
pub(crate) const KARO_EIGHT: BitCard = BitCard(2_u32.pow(1));
pub(crate) const KARO_SEVEN: BitCard = BitCard(2_u32.pow(0));


const fn get_binary_mask_for_rank(rank: u32) -> u32 {
    (1 << (21 + (rank - 1))) | (1 << (14 + (rank - 1))) | (1 << (7 + (rank - 1))) | (1 << (rank - 1))
}


const fn get_binary_mask_for_colors(lower: u32, upper: u32) -> u32 {
    ((1 << (upper - lower)) - 1) << lower
}




#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitCards(pub u32);


impl Debug for BitCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("BitCards");
        for card in *self {
            debug_struct.field("card", &card.get_human_representation());
        }
        debug_struct.finish()
    }
}

impl BitCards {
    ///Returns the next card in binary
    pub(crate) fn get_next_card_in_binary(&self) -> BitCard {
        BitCard(self.0 & (self.0 - 1) ^ self.0)
    }
}

impl BitOr for BitCards {
    type Output = BitCards;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitCards(self.0 | rhs.0)
    }
}

impl BitOr<BitCard> for BitCards {
    type Output = BitCards;

    fn bitor(self, rhs: BitCard) -> Self::Output {
        BitCards(self.0 | rhs.0)
    }
}

impl BitAnd for BitCards {
    type Output = BitCards;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitCards(self.0 & rhs.0)
    }
}

impl BitCards {
    pub(crate) fn get_cards_points(&self) -> u8 {
        let mut result = 0;
        for card in *self {
            result += card.get_point();
        }
        result
    }
}


impl Iterator for BitCards {
    type Item = BitCard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let next_card = self.get_next_card_in_binary();
        self.0 &= !next_card.0;
        Some(next_card)
    }
}

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct BitCard(pub u32);


impl Debug for BitCard {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitCard")
            .field("human_representation", &self.get_human_representation())
            .finish()
    }

}
impl BitOr for BitCard {
    type Output = BitCards;

    fn bitor(self, rhs: Self) -> BitCards {
        BitCards(self.0 | rhs.0)
    }
}

impl BitCard {
    fn get_numerical_representation(&self) -> u32 {
        self.0.ilog2()
        //CARDS_TO_INDEX[(self.0 % 37) as usize]
    }

    pub(crate) fn get_human_representation(&self) -> String {
        if self.0 == 0 {
            return "No card".to_string();
        }
        let numerical_representation = self.get_numerical_representation();

        match numerical_representation {
            28..32 => format!("Jack of {}", Self::calculate_which_suit_jack(numerical_representation - 28)),
            21..28 => format!("{} of Kreuz", Self::calculate_which_rank(numerical_representation - 21)),
            14..21 => format!("{} of Piqus", Self::calculate_which_rank(numerical_representation - 14)),
            7..14 => format!("{} of Hearts", Self::calculate_which_rank(numerical_representation - 7)),
            0..7 => format!("{} of Karo", Self::calculate_which_rank(numerical_representation)),
            _ => panic!("Should not happen")
        }
    }

    fn calculate_which_rank(rank: u32) -> &'static str {
        match rank {
            0 => "7",
            1 => "8",
            2 => "9",
            3 => "Dame",
            4 => "König",
            5 => "10",
            6 => "Ass",
            _ => panic!("Invalid argument: {}", rank)
        }
    }

    fn calculate_which_suit_jack(jack: u32) -> &'static str {
        match jack {
            0 => "Karo",
            1 => "Hearts",
            2 => "Piqus",
            3 => "Kreuz",
            _ => panic!("Should not happen")
        }
    }
    pub(crate) fn get_color_mask(&self) -> u32 {
        if self.0 & KREUZ_MASK != 0 {
            return KREUZ_MASK
        }
        else if self.0 & PIQUS_MASK != 0 {
            return PIQUS_MASK
        }
        else if self.0 & HEARTS_MASK != 0 {
            return HEARTS_MASK
        }
        KARO_MASK
    }
    pub(crate) fn get_point(&self) -> u8 {
        if self.0 & GRAND_MASK != 0 {
            return 2
        }
        if self.0 & SEVEN_OR_EIGHT_OR_NINE != 0 {
            return 0
        }
        if self.0 & TEN_MASK != 0 {
            return 10
        }
        if self.0 & QUEEN_MASK != 0 {
            return 3
        }
        if self.0 & KING_MASK != 0 {
            return 4
        }
        if self.0 & ACE_MASK != 0 {
            return 11
        }
        panic!("Should not happen")
    }

    pub fn greater_than(&self, other: BitCard, variant: &Variant) -> bool {
        let mask = variant.get_binary_mask();
        let card1 = self.0 & mask;
        let card2 = other.0 & mask;

        if card1 != 0 || card2 != 0 {
            return card1 > card2;
        }
        let other = other.0 & self.get_color_mask();
        self.0 > other
    }

}

#[inline(always)]
pub(crate) fn calculate_who_won_better(first : BitCard, second: BitCard, third: BitCard, variant: &Variant) -> (BitCard, u8) {
    let points = first.get_point() + second.get_point() + third.get_point();
    let mask = variant.get_binary_mask();
    let first_tr = first.0 & mask;
    let second_tr = second.0 & mask;
    let third_tr = third.0 & mask;
    if (first_tr | second_tr | third_tr) == 0 {
        let second_col = second.0 & first.get_color_mask();
        let thrid_col = third.0 & first.get_color_mask();
        return (BitCard(max(max(first.0, second_col), thrid_col)), points)
    }
    (BitCard(max(max(first_tr, second_tr), third_tr)), points)
}





#[cfg(test)]
mod tests {
    use crate::solver::bitboard::{calculate_who_won_better, HEARTS_EIGHT, HEARTS_QUEEN, HEARTS_TEN, KARO_EIGHT, KARO_NINE, KARO_SEVEN, KREUZ_ASS, KREUZ_EIGHT, KREUZ_KING, KREUZ_TEN, PIQUS_ASS, PIQUS_KING, PIQUS_TEN};
    use crate::solver::{Variant};

    #[test]
    fn cards_with_same_suit() {
        //calculates who won
        let winner = HEARTS_TEN;
        let loser_one = HEARTS_QUEEN;
        let loser_two = HEARTS_EIGHT;
        for variant in &[Variant::Clubs, Variant::Grand, Variant::Spades, Variant::Hearts, Variant::Diamonds] {
            let result = calculate_who_won_better(winner, loser_one, loser_two, variant);
            assert_eq!(result.0, winner);
            assert_eq!(result.1, 13);
        }

        let winner = KREUZ_ASS;
        let loser_one = KREUZ_KING;
        let loser_two = KREUZ_EIGHT;
        for variant in &[Variant::Clubs, Variant::Grand, Variant::Spades, Variant::Hearts, Variant::Diamonds] {
            let result = calculate_who_won_better(winner, loser_one, loser_two, variant);
            assert_eq!(result.0, winner);
            assert_eq!(result.1, 15);
        }
        let winner = PIQUS_ASS;
        let loser_one = PIQUS_TEN;
        let loser_two = PIQUS_KING;
        for variant in &[Variant::Clubs, Variant::Grand, Variant::Spades, Variant::Hearts, Variant::Diamonds] {
            let result = calculate_who_won_better(loser_two, loser_one, winner, variant);
            assert_eq!(result.0, winner);
            assert_eq!(result.1, 25);
        }

        let winner = KARO_NINE;
        let loser_one = KARO_SEVEN;
        let loser_two = KARO_EIGHT;
        for variant in &[Variant::Clubs, Variant::Grand, Variant::Spades, Variant::Hearts, Variant::Diamonds] {
            let result = calculate_who_won_better(loser_two, loser_one, winner, variant);
            assert_eq!(result.0, winner);
            assert_eq!(result.1, 0);
        }
    }

    #[test]
    fn cards_with_one_trumpf() {
        let winner = KREUZ_TEN;
        let loser_one = PIQUS_ASS;
        let loser_two = PIQUS_KING;
        let result = calculate_who_won_better(loser_two, loser_one, winner, &Variant::Clubs);
        assert_eq!(result.0, winner);
        assert_eq!(result.1, 25);

    }
}