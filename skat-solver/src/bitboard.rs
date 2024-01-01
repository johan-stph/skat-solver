use std::cmp::Ordering;
use std::ops::{BitAnd, BitOr};

const CARDS_TO_INDEX: [u32; 37] = [
    u32::MAX, 0, 1, 26, 2, 23, 27, u32::MAX, 3, 16, 24, 30, 28, 11, u32::MAX, 13, 4, 7, 17, u32::MAX, 25, 22, 31, 15, 29, 10, 12, 6, u32::MAX, 21, 14, 9, 5, 20, 8, 19, 18];

const GRAND_MASK : u32 = !((1 << 28) - 1);
const KREUZ_MASK: u32 = get_binary_mask_for_colors(21, 28);
const PIQUS_MASK : u32 = get_binary_mask_for_colors(14, 21);
const HEARTS_MASK : u32 = get_binary_mask_for_colors(7, 14);
const KARO_MASK: u32 = get_binary_mask_for_colors(0, 7);

const KREUZ_TRUMPF_MASK: u32 = KREUZ_MASK | GRAND_MASK;
const PIQUS_TRUMPF_MASK: u32 = PIQUS_MASK | GRAND_MASK;
const HEARTS_TRUMPF_MASK: u32 = HEARTS_MASK | GRAND_MASK;
const KARO_TRUMPF_MASK: u32 = KARO_MASK | GRAND_MASK;

const SEVEN_MASK : u32 = get_binary_mask_for_rank(1);
const EIGHT_MASK : u32 = get_binary_mask_for_rank(2);
const NINE_MASK : u32 = get_binary_mask_for_rank(3);
const QUEEN_MASK : u32 = get_binary_mask_for_rank(4);
const KING_MASK : u32 = get_binary_mask_for_rank(5);
const TEN_MASK : u32 = get_binary_mask_for_rank(6);
const ACE_MASK : u32 = get_binary_mask_for_rank(7);


const fn get_binary_mask_for_rank(rank: u32) -> u32 {
    (1 << (21 + (rank - 1))) | (1 << (14 + (rank - 1))) | (1 << (7 + (rank - 1))) | (1 << (rank - 1))
}



const fn get_binary_mask_for_colors(lower: u32, upper: u32) -> u32 {
    ((1 << (upper - lower)) - 1) << lower
}


#[derive(Debug)]
pub enum Variant {
    Grand,
    NullHand,
    NullOuvert,
    NullOuvertHand,
    Null,
    Diamonds,
    Hearts,
    Spades,
    Clubs,
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
                PIQUS_MASK | GRAND_MASK
            }
            Variant::Clubs => {
                KREUZ_MASK | GRAND_MASK
            }
            _ => unimplemented!("Not implemented yet")
        }

    }

}


#[derive(Debug, Copy, Clone)]
pub struct BitCards(pub(crate) u32);

impl BitCards {

    ///Returns the next card in binary
    fn get_next_card_in_binary(&self) -> BitCard {
        BitCard(self.0 & (self.0 - 1) ^ self.0)
    }
}

impl BitOr for BitCards {
    type Output = BitCards;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitCards(self.0 | rhs.0)
    }
}
impl BitAnd for BitCards {
    type Output = BitCards;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitCards(self.0 & rhs.0)
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BitCard(pub(crate) u32);


impl BitCard {
    fn get_numerical_representation(&self) -> u32 {
        self.0.ilog2()
        //CARDS_TO_INDEX[(self.0 % 37) as usize]
    }

    pub(crate) fn get_human_representation(&self) -> String {
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

    pub fn greater_than(&self, other: &BitCard, variant: &Variant) -> bool {
        let mask = variant.get_binary_mask();
        let card1 = self.0 & mask;
        let card2 = other.0 & mask;

        if card1 != 0 || card2 != 0 {
            return card1 > card2;
        }
        let other = other.0 & self.get_color_mask();
        self.0 > other
    }

    pub(crate) fn get_point(&self) -> u32 {
        if self.0 & GRAND_MASK != 0 {
            return 2
        }
        if self.0 & (SEVEN_MASK | EIGHT_MASK | NINE_MASK) != 0 {
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
}


#[cfg(test)]
mod tests {
    use crate::bitboard::{ACE_MASK, BitCards, KING_MASK, NINE_MASK, PIQUS_MASK, QUEEN_MASK, SEVEN_MASK, TEN_MASK, Variant};

    #[test]
    fn test_cards() {
        //that

        let variant_hearts = Variant::Grand;

        let mut first_card : BitCards = BitCards(u32::MAX);

        for _ in 0..32 {
            let next_card_1 = first_card.get_next_card_in_binary();

            let mut second_card = BitCards(u32::MAX);

            for _ in 0..32 {
                let next_card_2 = second_card.get_next_card_in_binary();
                if next_card_1 != next_card_2 {
                    if next_card_1.greater_than(&next_card_2, &variant_hearts) {
                        println!("{} > {}", next_card_1.get_human_representation(), next_card_2.get_human_representation());
                    } else {
                        println!("{} < {}", next_card_1.get_human_representation(), next_card_2.get_human_representation());

                    }
                }

                second_card = BitCards(second_card.0 & !next_card_2.0)
            }
            first_card = BitCards(first_card.0 & (!next_card_1.0));

        }
    }

    #[test]
    fn test_mask() {
        let mut sum: u32 = 0;
        for card in BitCards(u32::MAX) {
            sum += card.get_point();
        }
        assert_eq!(sum, 120);
    }
}