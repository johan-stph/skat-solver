#![allow(dead_code)]

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::Suit::{Kreuz, Karo, Heart, Piqus};



mod game;

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
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
            Rank::Ace => {
                if other == &Rank::Ace {
                    Equal
                } else {
                    Greater
                }
            }
            Rank::Eight => {
                match other {
                    Rank::Eight => Equal,
                    Rank::Seven => Greater,
                    _ => Less,
                }
            }
            Rank::King => {
                match other {
                    Rank::King => Equal,
                    Rank::Ace => Less,
                    _ => Greater,
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
                    Rank::Ace | Rank::King => Greater,
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
                    Rank::Ace | Rank::King | Rank::Queen => Greater,
                    _ => Less,
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



#[derive(Debug)]
enum Variant {
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

fn determine_who_won_round(card1: &Card, card2: &Card, card3: &Card, game: &Variant) -> u32 {
    match game {
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
    }
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
        let winner = determine_who_won_round(&card1, &card2, &card3, &variant);
        checksum += card1.rank.value() + card2.rank.value() + card3.rank.value();
        assert_eq!(checksum, 21);
        assert_eq!(winner, 2);
        // game 2
        let card4 = Card { suit: Piqus, rank: Rank::Ace };
        let card5 = Card { suit: Piqus, rank: Rank::Seven };
        let card6 = Card { suit: Piqus, rank: Rank::Ten };
        let winner = determine_who_won_round(&card4, &card5, &card6, &variant);
        checksum += card4.rank.value() + card5.rank.value() + card6.rank.value();
        assert_eq!(checksum, 42);
        assert_eq!(winner, 0);
        // game 3
        let card7 = Card { suit: Piqus, rank: Rank::Jack };
        let card8 = Card { suit: Heart, rank: Rank::Ace };
        let card9 = Card { suit: Kreuz, rank: Rank::Jack };
        let winner = determine_who_won_round(&card7, &card8, &card9, &variant);
        checksum += card7.rank.value() + card8.rank.value() + card9.rank.value();
        assert_eq!(winner, 2);
        assert_eq!(checksum, 57);
        // game 4
        let card10 = Card { suit: Karo, rank: Rank::Nine };
        let card11 = Card { suit: Heart, rank: Rank::Eight };
        let card12 = Card { suit: Karo, rank: Rank::Queen };
        let winner = determine_who_won_round(&card10, &card11, &card12, &variant);
        checksum += card10.rank.value() + card11.rank.value() + card12.rank.value();
        assert_eq!(checksum, 60);
        assert_eq!(winner, 1);
        // game 5
        let card13 = Card { suit: Piqus, rank: Rank::Queen };
        let card14 = Card { suit: Piqus, rank: Rank::Nine };
        let card15 = Card { suit: Heart, rank: Rank::Nine };
        let winner = determine_who_won_round(&card13, &card14, &card15, &variant);
        checksum += card13.rank.value() + card14.rank.value() + card15.rank.value();
        assert_eq!(checksum, 63);
        assert_eq!(winner, 2);
        // game 6
        let card16 = Card { suit: Karo, rank: Rank::King };
        let card17 = Card { suit: Heart, rank: Rank::Queen };
        let card18 = Card { suit: Karo, rank: Rank::Ten };
        let winner = determine_who_won_round(&card16, &card17, &card18, &variant);
        checksum += card16.rank.value() + card17.rank.value() + card18.rank.value();
        assert_eq!(checksum, 80);
        assert_eq!(winner, 1);
        // game 7
        let card19 = Card { suit: Piqus, rank: Rank::King };
        let card20 = Card { suit: Kreuz, rank: Rank::Ace };
        let card21 = Card { suit: Heart, rank: Rank::Seven };
        let winner = determine_who_won_round(&card19, &card20, &card21, &variant);
        checksum += card19.rank.value() + card20.rank.value() + card21.rank.value();
        assert_eq!(checksum, 95);
        assert_eq!(winner, 2);
        // game 8
        let card22 = Card { suit: Kreuz, rank: Rank::Queen };
        let card23 = Card { suit: Heart, rank: Rank::King };
        let card24 = Card { suit: Kreuz, rank: Rank::Nine };
        let winner = determine_who_won_round(&card22, &card23, &card24, &variant);
        checksum += card22.rank.value() + card23.rank.value() + card24.rank.value();
        assert_eq!(checksum, 102);
        assert_eq!(winner, 1);
        // game 9
        let card25 = Card { suit: Piqus, rank: Rank::Eight };
        let card26 = Card { suit: Kreuz, rank: Rank::Ten };
        let card27 = Card { suit: Kreuz, rank: Rank::Seven };
        let winner = determine_who_won_round(&card25, &card26, &card27, &variant);
        checksum += card25.rank.value() + card26.rank.value() + card27.rank.value();
        assert_eq!(checksum, 112);
        assert_eq!(winner, 0);

        // game 10
        let card28 = Card { suit: Karo, rank: Rank::Jack };
        let card29 = Card { suit: Heart, rank: Rank::Jack };
        let card30 = Card { suit: Kreuz, rank: Rank::King };
        let winner = determine_who_won_round(&card28, &card29, &card30, &variant);
        checksum += card28.rank.value() + card29.rank.value() + card30.rank.value();
        assert_eq!(checksum, 120);
        assert_eq!(winner, 1);

        let card31 = Card { suit: Karo, rank: Rank::Eight };
        let card32 = Card { suit: Kreuz, rank: Rank::Eight };
        checksum += card31.rank.value() + card32.rank.value();
        assert_eq!(checksum, 120);
    }
}


