#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use crate::bitboard::Variant;
use crate::Suit::{Kreuz, Karo, Heart, Piqus};



mod solver;
mod bitboard;

pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?},{:?}", self.suit, self.rank)
    }
}

impl Card {

    fn null_comparison(&self, other: &Card) -> Ordering {
        if self.suit != other.suit {
            return Greater;
        }
        self.rank.null_ordering(&other.rank)
    }
    fn compare_jacks(&self, other: &Card) -> Option<Ordering> {
        if self.rank == Rank::Jack || other.rank == Rank::Jack {
            if other.rank != Rank::Jack {
                return Some(Greater);
            }
            if self.rank != Rank::Jack {
                return Some(Less);
            }
            return Some(self.suit.order_jacks(&other.suit));
        }
        None
    }

    fn grand_comparison(&self, other: &Card) -> Ordering {
        if let Some(order) = self.compare_jacks(other) {
            return order;
        }
        if self.suit != other.suit {
            return Greater;
        }
        self.rank.grand_ordering(&other.rank)
    }

    fn color_comparison(&self, other: &Card, trumpf: &Suit) -> Ordering {
        if let Some(order) = self.compare_jacks(other) {
            return order;
        }
        if self.suit == *trumpf && other.suit != *trumpf {
            return Greater;
        }
        if self.suit != *trumpf && other.suit == *trumpf {
            return Less;
        }
        if self.suit != other.suit {
            return Greater;
        }
        self.rank.grand_ordering(&other.rank)
    }

}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Rank {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn value(&self) -> u32 {
        match self {
            Rank::Seven => 0,
            Rank::Eight => 0,
            Rank::Nine => 0,
            Rank::Ten => 10,
            Rank::Jack => 2,
            Rank::Queen => 3,
            Rank::King => 4,
            Rank::Ace => 11,
        }
    }

    fn null_ordering(&self, other: &Rank) -> Ordering {
        match self {
            Rank::Seven => {
                if other == &Rank::Seven {
                    Equal
                } else {
                    Less
                }
            }

            Rank::Eight => {
                match other {
                    Rank::Eight => Equal,
                    Rank::Seven => Greater,
                    _ => Less,
                }
            }

            Rank::Nine => {
                match other {
                    Rank::Nine => Equal,
                    Rank::Seven | Rank::Eight => Greater,
                    _ => Less,
                }
            }

            Rank::Ten => {
                match other {
                    Rank::Ten => Equal,
                    Rank::Seven | Rank::Eight | Rank::Nine => Greater,
                    _ => Less,
                }
            }

            Rank::Jack => {
                match other {
                    Rank::Jack => Equal,
                    Rank::Ace | Rank::King | Rank::Queen => Less,
                    _ => Greater,
                }
            }
            Rank::Queen => {
                match other {
                    Rank::Queen => Equal,
                    Rank::Ace | Rank::King => Less,
                    _ => Greater,
                }
            }

            Rank::King => {
                match other {
                    Rank::King => Equal,
                    Rank::Ace => Less,
                    _ => Greater,
                }
            }

            Rank::Ace => {
                if other == &Rank::Ace {
                    Equal
                } else {
                    Greater
                }
            }
        }
    }

    fn grand_ordering(&self, other: &Rank) -> Ordering {
        match self {
            Rank::Seven => {
                match other {
                    Rank::Seven => Equal,
                    _ => Less,
                }
            }
            Rank::Eight => {
                match other {
                    Rank::Eight => Equal,
                    Rank::Seven => Greater,
                    _ => Less,
                }
            }
            Rank::Nine => {
                match other {
                    Rank::Nine => Equal,
                    Rank::Seven | Rank::Eight => Greater,
                    _ => Less,
                }
            }
            Rank::Queen => {
                match other {
                    Rank::Queen => Equal,
                    Rank::Seven | Rank::Eight | Rank::Nine => Greater,
                    _ => Less,
                }
            }
            Rank::King => {
                match other {
                    Rank::King => Equal,
                    Rank::Ace | Rank::Ten => Less,
                    _ => Greater,
                }
            }
            Rank::Ten => {
                match other {
                    Rank::Ten => Equal,
                    Rank::Ace => Less,
                    _ => Greater,
                }
            }
            Rank::Ace => {
                match other {
                    Rank::Ace => Equal,
                    _ => Greater,
                }
            }
            Rank::Jack => panic!("Jack should not be compared in grand"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Suit {
    Kreuz,
    Piqus,
    Heart,
    Karo,
}

impl Suit {

    fn order_jacks(&self, other: &Suit) -> Ordering {
        match self {
            Kreuz => {
                Greater
            }
            Piqus => {
                if other == &Kreuz {
                    Less
                }
                else {
                    Greater
                }
            }
            Heart => {
                if other == &Karo {
                    Greater
                } else {
                    Less
                }
            }
            Karo => {
                Less
            }
        }
    }
}





impl TryInto<Suit> for &Variant {
    type Error = ();

    fn try_into(self) -> Result<Suit, Self::Error> {
        match self {
            Variant::Diamonds => Ok(Karo),
            Variant::Hearts => Ok(Heart),
            Variant::Spades => Ok(Piqus),
            Variant::Clubs => Ok(Kreuz),
            _ => Err(()),
        }
    }
}


pub fn can_be_placed<'a>(first_card: &Card, remaining_cards: &'a [Card], game: &Variant) -> Vec<&'a Card> {
    match game {
        Variant::Null | Variant::NullHand | Variant::NullOuvert | Variant::NullOuvertHand => {
            let playable_cards: Vec<&Card> = remaining_cards.iter()
                .filter(|&card| card.suit == first_card.suit)
                .collect();
            if playable_cards.is_empty() {
                remaining_cards.iter().collect()
            } else {
                playable_cards
            }
        }
        Variant::Grand => {
            // Implement the logic for Variant::Grand
            if first_card.rank == Rank::Jack {
                let playabale_cards: Vec<&Card> = remaining_cards.iter()
                    .filter(|&card| card.rank == Rank::Jack)
                    .collect();
                return if playabale_cards.is_empty() {
                    remaining_cards.iter().collect()
                } else {
                    playabale_cards
                }
            }
            let playabale_cards: Vec<&Card> = remaining_cards.iter()
                .filter(|&card| card.suit == first_card.suit)
                .collect();
            return if playabale_cards.is_empty() {
                remaining_cards.iter().collect()
            } else {
                playabale_cards
            }
        }
        Variant::Diamonds | Variant::Hearts | Variant::Spades | Variant::Clubs => {
            // Implement the logic for the other variants
            let trumpf: Suit = game.try_into().unwrap();
            if first_card.rank == Rank::Jack || first_card.suit == trumpf {
                let playabale_cards: Vec<&Card> = remaining_cards.iter()
                    .filter(|&card| card.rank == Rank::Jack || card.suit == trumpf)
                    .collect();
                return if playabale_cards.is_empty() {
                    remaining_cards.iter().collect()
                } else {
                    playabale_cards
                }
            }
            let playabale_cards: Vec<&Card> = remaining_cards.iter()
                .filter(|&card| card.suit == first_card.suit && card.rank != Rank::Jack)
                .collect();
            return if playabale_cards.is_empty() {
                remaining_cards.iter().collect()
            } else {
                playabale_cards
            }
        }
    }
}



pub fn determine_who_won_round(card1: &Card, card2: &Card, card3: &Card, game: &Variant) -> (u32, u32) {
    let value = card1.rank.value() + card2.rank.value() + card3.rank.value();
    let first: u32 = match game {
        Variant::Null | Variant::NullOuvert | Variant::NullOuvertHand | Variant::NullHand => {
            match card1.null_comparison(card2) {
                Greater => {
                    match card1.null_comparison(card3) {
                        Greater => 0,
                        Equal => panic!("Two cards cannot be equal in null"),
                        Less => 2,
                    }
                }
                Less => {
                    match card2.null_comparison(card3) {
                        Greater => 1,
                        Equal => panic!("Two cards cannot be equal in null"),
                        Less => 2,
                    }
                }
                Equal => {
                    panic!("Two cards cannot be equal in null")
                }
            }

        },
        Variant::Grand => {
            match card1.grand_comparison(card2) {
                Greater => {
                    match card1.grand_comparison(card3) {
                        Greater => 0,
                        Equal => panic!("Two cards cannot be equal in grand"),
                        Less => 2,
                    }
                }
                Less => {
                    match card2.grand_comparison(card3) {
                        Greater => 1,
                        Equal => panic!("Two cards cannot be equal in grand"),
                        Less => 2,
                    }
                }
                Equal => {
                    panic!("Two cards cannot be equal in grand")
                }
            }
        },
        Variant::Spades | Variant::Hearts | Variant::Diamonds | Variant::Clubs => {
            let trumpf: Suit = game.try_into().unwrap();
            match card1.color_comparison(card2, &trumpf) {
                Greater => {
                    match card1.color_comparison(card3, &trumpf) {
                        Greater => 0,
                        Equal => panic!("Two cards cannot be equal in color"),
                        Less => 2,
                    }
                }
                Less => {
                    match card2.color_comparison(card3, &trumpf) {
                        Greater => 1,
                        Equal => panic!("Two cards cannot be equal in color"),
                        Less => 2,
                    }
                }
                Equal => {
                    panic!("Two cards cannot be equal in color")
                }
            }
        }
    };
    (first, value)
}


struct BoardPos {

    game: Variant,
    declarer: Vec<Card>,
    opponent1: Vec<Card>,
    opponent2: Vec<Card>,
    skat: Vec<Card>,
    points: u32,
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determine_winner_hearts() {
        let variant: Variant = Variant::Hearts;
        let mut checksum = 0_u32;
        // game 1
        let card1 = Card { suit: Karo, rank: Rank::Ace };
        let card2 = Card { suit: Karo, rank: Rank::Seven };
        let card3 = Card { suit: Heart, rank: Rank::Ten };
        let (winner, points) = determine_who_won_round(&card1, &card2, &card3, &variant);
        checksum += points;
        assert_eq!(checksum, 21);
        assert_eq!(winner, 2);
        // game 2
        let card4 = Card { suit: Piqus, rank: Rank::Ace };
        let card5 = Card { suit: Piqus, rank: Rank::Seven };
        let card6 = Card { suit: Piqus, rank: Rank::Ten };
        let (winner, points) = determine_who_won_round(&card4, &card5, &card6, &variant);
        checksum += points;
        assert_eq!(checksum, 42);
        assert_eq!(winner, 0);
        // game 3
        let card7 = Card { suit: Piqus, rank: Rank::Jack };
        let card8 = Card { suit: Heart, rank: Rank::Ace };
        let card9 = Card { suit: Kreuz, rank: Rank::Jack };
        let (winner, points) = determine_who_won_round(&card7, &card8, &card9, &variant);
        checksum += points;
        assert_eq!(winner, 2);
        assert_eq!(checksum, 57);
        // game 4
        let card10 = Card { suit: Karo, rank: Rank::Nine };
        let card11 = Card { suit: Heart, rank: Rank::Eight };
        let card12 = Card { suit: Karo, rank: Rank::Queen };
        let (winner, points) = determine_who_won_round(&card10, &card11, &card12, &variant);
        checksum += points;
        assert_eq!(checksum, 60);
        assert_eq!(winner, 1);
        // game 5
        let card13 = Card { suit: Piqus, rank: Rank::Queen };
        let card14 = Card { suit: Piqus, rank: Rank::Nine };
        let card15 = Card { suit: Heart, rank: Rank::Nine };
        let (winner, points) = determine_who_won_round(&card13, &card14, &card15, &variant);
        checksum += points;
        assert_eq!(checksum, 63);
        assert_eq!(winner, 2);
        // game 6
        let card16 = Card { suit: Karo, rank: Rank::King };
        let card17 = Card { suit: Heart, rank: Rank::Queen };
        let card18 = Card { suit: Karo, rank: Rank::Ten };
        let (winner, points) = determine_who_won_round(&card16, &card17, &card18, &variant);
        checksum += points;
        assert_eq!(checksum, 80);
        assert_eq!(winner, 1);
        // game 7
        let card19 = Card { suit: Piqus, rank: Rank::King };
        let card20 = Card { suit: Kreuz, rank: Rank::Ace };
        let card21 = Card { suit: Heart, rank: Rank::Seven };
        let (winner, points) = determine_who_won_round(&card19, &card20, &card21, &variant);
        checksum += points;
        assert_eq!(checksum, 95);
        assert_eq!(winner, 2);
        // game 8
        let card22 = Card { suit: Kreuz, rank: Rank::Queen };
        let card23 = Card { suit: Heart, rank: Rank::King };
        let card24 = Card { suit: Kreuz, rank: Rank::Nine };
        let (winner, points) = determine_who_won_round(&card22, &card23, &card24, &variant);
        checksum += points;
        assert_eq!(checksum, 102);
        assert_eq!(winner, 1);
        // game 9
        let card25 = Card { suit: Piqus, rank: Rank::Eight };
        let card26 = Card { suit: Kreuz, rank: Rank::Ten };
        let card27 = Card { suit: Kreuz, rank: Rank::Seven };
        let (winner, points) = determine_who_won_round(&card25, &card26, &card27, &variant);
        checksum += points;
        assert_eq!(checksum, 112);
        assert_eq!(winner, 0);

        // game 10
        let card28 = Card { suit: Karo, rank: Rank::Jack };
        let card29 = Card { suit: Heart, rank: Rank::Jack };
        let card30 = Card { suit: Kreuz, rank: Rank::King };
        let (winner, points) = determine_who_won_round(&card28, &card29, &card30, &variant);
        checksum += points;
        assert_eq!(checksum, 120);
        assert_eq!(winner, 1);

        let card31 = Card { suit: Karo, rank: Rank::Eight };
        let card32 = Card { suit: Kreuz, rank: Rank::Eight };
        checksum += card31.rank.value() + card32.rank.value();
        assert_eq!(checksum, 120);
    }

    #[test]
    fn determine_winner_null() {
        let variant: Variant = Variant::Null;
        // game 1
        let card1 = Card { suit: Karo, rank: Rank::Jack };
        let card2 = Card { suit: Karo, rank: Rank::Ace  };
        let card3 = Card { suit: Heart, rank: Rank::Nine };
        let (winner, _) = determine_who_won_round(&card1, &card2, &card3, &variant);
        assert_eq!(winner, 1);

        // game 2
        let card4 = Card { suit: Piqus, rank: Rank::Seven };
        let card5 = Card { suit: Piqus, rank: Rank::Eight };
        let card6 = Card { suit: Piqus, rank: Rank::Ace };
        let (winner, _) = determine_who_won_round(&card4, &card5, &card6, &variant);
        assert_eq!(winner, 2);

        // game 3
        let card7 = Card { suit: Kreuz, rank: Rank::Jack };
        let card8 = Card { suit: Kreuz, rank: Rank::King };
        let card9 = Card { suit: Kreuz, rank: Rank::Seven };
        let (winner, _) = determine_who_won_round(&card7, &card8, &card9, &variant);
        assert_eq!(winner, 1);

        // game 4
        let card10 = Card { suit: Kreuz, rank: Rank::Nine };
        let card11 = Card { suit: Heart, rank: Rank::Jack };
        let card12 = Card { suit: Kreuz, rank: Rank::Eight };
        let (winner, _) = determine_who_won_round(&card10, &card11, &card12, &variant);
        assert_eq!(winner, 0);

        // game 5
        let card13 = Card { suit: Piqus, rank: Rank::Nine };
        let card14 = Card { suit: Heart, rank: Rank::Ten };
        let card15 = Card { suit: Piqus, rank: Rank::Queen };
        let (winner, _) = determine_who_won_round(&card13, &card14, &card15, &variant);
        assert_eq!(winner, 2);

        // game 6
        let card16 = Card { suit: Heart, rank: Rank::Queen };
        let card17 = Card { suit: Heart, rank: Rank::Seven };
        let card18 = Card { suit: Heart, rank: Rank::Eight };
        let (winner, _) = determine_who_won_round(&card16, &card17, &card18, &variant);
        assert_eq!(winner, 0);

    }

    #[test]
    fn determine_winner_grand() {
        let variant: Variant = Variant::Grand;
        let mut checksum = 0_u32;
        // game 1
        let card1 = Card { suit: Piqus, rank: Rank::Seven };
        let card2 = Card { suit: Piqus, rank: Rank::Queen };
        let card3 = Card { suit: Piqus, rank: Rank::Ace };
        let (winner, points) = determine_who_won_round(&card1, &card2, &card3, &variant);
        checksum += points;
        assert_eq!(checksum, 14);
        assert_eq!(winner, 2);
        // game 2
        let card4 = Card { suit: Kreuz, rank: Rank::Ace };
        let card5 = Card { suit: Kreuz, rank: Rank::Seven };
        let card6 = Card { suit: Kreuz, rank: Rank::King };
        let (winner, points) = determine_who_won_round(&card4, &card5, &card6, &variant);
        checksum += points;
        assert_eq!(checksum, 29);
        assert_eq!(winner, 0);
        // game 3
        let card7 = Card { suit: Heart, rank: Rank::Ace };
        let card8 = Card { suit: Heart, rank: Rank::Queen };
        let card9 = Card { suit: Heart, rank: Rank::Eight };
        let (winner, points) = determine_who_won_round(&card7, &card8, &card9, &variant);
        checksum += points;
        assert_eq!(winner, 0);
        assert_eq!(checksum, 43);
        // game 4
        let card10 = Card { suit: Karo, rank: Rank::Ace };
        let card11 = Card { suit: Karo, rank: Rank::Eight };
        let card12 = Card { suit: Karo, rank: Rank::Seven };
        let (winner, points) = determine_who_won_round(&card10, &card11, &card12, &variant);
        checksum += points;
        assert_eq!(checksum, 54);
        assert_eq!(winner, 0);
        // game 5
        let card13 = Card { suit: Piqus, rank: Rank::Ten };
        let card14 = Card { suit: Piqus, rank: Rank::Jack };
        let card15 = Card { suit: Piqus, rank: Rank::Eight };
        let (winner, points) = determine_who_won_round(&card13, &card14, &card15, &variant);
        checksum += points;
        assert_eq!(checksum, 66);
        assert_eq!(winner, 1);
        // game 6
        let card16 = Card { suit: Kreuz, rank: Rank::Eight };
        let card17 = Card { suit: Heart, rank: Rank::Jack };
        let card18 = Card { suit: Kreuz, rank: Rank::Queen };
        let (winner, points) = determine_who_won_round(&card16, &card17, &card18, &variant);
        checksum += points;
        assert_eq!(checksum, 71);
        assert_eq!(winner, 1);
        // game 7
        let card19 = Card { suit: Piqus, rank: Rank::King };
        let card20 = Card { suit: Karo, rank: Rank::Jack };
        let card21 = Card { suit: Karo, rank: Rank::Nine };
        let (winner, points) = determine_who_won_round(&card19, &card20, &card21, &variant);
        checksum += points;
        assert_eq!(checksum, 77);
        assert_eq!(winner, 1);
        // game 8
        let card22 = Card { suit: Kreuz, rank: Rank::Ten };
        let card23 = Card { suit: Kreuz, rank: Rank::Nine };
        let card24 = Card { suit: Heart, rank: Rank::Nine };
        let (winner, points) = determine_who_won_round(&card22, &card23, &card24, &variant);
        checksum += points;
        assert_eq!(checksum, 87);
        assert_eq!(winner, 0);
        // game 9
        let card25 = Card { suit: Karo, rank: Rank::Queen };
        let card26 = Card { suit: Kreuz, rank: Rank::Jack };
        let card27 = Card { suit: Karo, rank: Rank::King };
        let (winner, points) = determine_who_won_round(&card25, &card26, &card27, &variant);
        checksum += points;
        assert_eq!(checksum, 96);
        assert_eq!(winner, 1);

        // game 10
        let card28 = Card { suit: Heart, rank: Rank::King };
        let card29 = Card { suit: Piqus, rank: Rank::Nine };
        let card30 = Card { suit: Heart, rank: Rank::Seven };
        let (winner, points) = determine_who_won_round(&card28, &card29, &card30, &variant);
        checksum += points;
        assert_eq!(checksum, 100);
        assert_eq!(winner, 0);

        let card31 = Card { suit: Karo, rank: Rank::Ten };
        let card32 = Card { suit: Heart, rank: Rank::Ten };
        checksum += card31.rank.value() + card32.rank.value();
        assert_eq!(checksum, 120);

    }

    #[test]
    fn can_be_placed_hearts() {
        let variant: Variant = Variant::Hearts;
        let mut cards1 = vec![
            Card { suit: Heart, rank: Rank::Jack },
            Card { suit: Heart, rank: Rank::Ace },
            Card { suit: Kreuz, rank: Rank::Ace },
            Card { suit: Kreuz, rank: Rank::Ten  },
            Card { suit: Kreuz, rank: Rank::Nine },
            Card { suit: Piqus, rank: Rank::Nine },
            Card { suit: Piqus, rank: Rank::Seven },
            Card { suit: Karo, rank: Rank::Ace },
            Card { suit: Karo, rank: Rank::Ten },
            Card { suit: Karo, rank: Rank::Queen },
        ];
        let mut cards2 = vec![
            Card { suit: Kreuz, rank: Rank::Jack },
            Card { suit: Heart, rank: Rank::Nine },
            Card { suit: Heart, rank: Rank::Seven },
            Card { suit: Kreuz, rank: Rank::King  },
            Card { suit: Kreuz, rank: Rank::Queen },
            Card { suit: Kreuz, rank: Rank::Seven },
            Card { suit: Piqus, rank: Rank::Ten },
            Card { suit: Karo, rank: Rank::King },
            Card { suit: Karo, rank: Rank::Nine },
            Card { suit: Karo, rank: Rank::Seven },
        ];
        let mut cards3 = vec![
            Card { suit: Piqus, rank: Rank::Jack },
            Card { suit: Karo, rank: Rank::Jack },
            Card { suit: Heart, rank: Rank::Ten },
            Card { suit: Heart, rank: Rank::King },
            Card { suit: Heart, rank: Rank::Queen },
            Card { suit: Heart, rank: Rank::Eight },
            Card { suit: Piqus, rank: Rank::Ace },
            Card { suit: Piqus, rank: Rank::King },
            Card { suit: Piqus, rank: Rank::Queen },
            Card { suit: Piqus, rank: Rank::Eight },
        ];
        let playable = can_be_placed(&cards1[7], &cards2, &variant);
        assert_eq!(playable.len(), 3);
        let playable = can_be_placed(&cards1[7], &cards3, &variant);
        assert_eq!(playable.len(), 10);
        cards1.remove(7);
        cards2.remove(9);
        cards3.remove(2);
        let playable = can_be_placed(&cards3[5], &cards1, &variant);
        assert_eq!(playable.len(), 2);
        let playable = can_be_placed(&cards3[5], &cards2, &variant);
        assert_eq!(playable.len(), 1);

    }

    #[test]
    fn simulate_game() {

    }

}


