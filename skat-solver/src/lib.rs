use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::Suit::{Clubs, Diamonds};

mod game;



pub trait GameCommands {
    fn start_game(&mut self);
    fn get_score(&self) -> (u32, u32, u32);
    fn finish_game(&mut self);

    fn help(&self);

}

pub trait Input {
    fn get_input(&self) -> String;
}

pub trait Output {
    fn display(&self, message: &str);
}

struct Player {
    points: u32,
    hand: Vec<Card>,
    name: String,
}
#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {

    fn null_comparison(&self, other: &Card) -> Ordering {
        if self.suit != other.suit {
            return Ordering::Greater;
        }
        self.rank.null_ordering(&other.rank)
    }

    fn grand_comparison(&self, other: &Card) -> Ordering {
        if self.rank == Rank::Jack || other.rank == Rank::Jack {
            if other.rank != Rank::Jack {
                return Ordering::Greater;
            }
            if self.rank != Rank::Jack {
                return Less;
            }
            return self.suit.order_jacks(&other.suit);
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
    fn null_ordering(&self, other: &Rank) -> Ordering {
        todo!()
    }

    fn grand_ordering(&self, other: &Rank) -> Ordering {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl Suit {

    fn order_jacks(&self, other: &Suit) -> Ordering {
        match self {
            Suit::Clubs => {
                Greater
            }
            Suit::Spades => {
                if other == &Clubs {
                    Less
                }
                else {
                    Greater
                }
            }
            Suit::Hearts => {
                if other == &Diamonds {
                    Greater
                } else {
                    Less
                }
            }
            Suit::Diamonds => {
                Less
            }
        }
    }
}

pub struct Game {

    players: (Player, Player, Player),
    input: Box<dyn Input>,
    output: Box<dyn Output>,
}

impl Game {
    pub fn new(input: Box<dyn Input>, output: Box<dyn Output>) -> Self {
        output.display("Starting game!");
        output.display("Enter the player names (Seperated by new lines): ");
        let first_name = input.get_input();
        let second_name = input.get_input();
        let third_name = input.get_input();
        output.display(&format!("Player 1: {}\nPlayer 2: {}\nPlayer 3: {}", first_name, second_name, third_name));
        Self {
            players: (Player {
                points: 0,
                hand: Vec::new(),
                name: first_name,
            }, Player {
                points: 0,
                hand: Vec::new(),
                name: second_name,
            }, Player {
                points: 0,
                hand: Vec::new(),
                name: third_name,
            }),
            input,
            output,
        }
    }
}

impl GameCommands for Game {
    fn start_game(&mut self) {
        let mut hand1 = game::generate_card_deck();
        let mut hand2 = hand1.split_off(10);
        let mut hand3 = hand2.split_off(10);
        let skat = hand3.split_off(10);
        self.players.0.hand = hand1;
        self.players.1.hand = hand2;
        self.players.2.hand = hand3;

    }

    fn get_score(&self) -> (u32, u32, u32) {
        (self.players.0.points, self.players.1.points, self.players.2.points)
    }

    fn finish_game(&mut self) {
        todo!()
    }

    fn help(&self) {
        self.output.display("Commands:\n\
        help - Display this message");
    }
}

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



fn determine_who_round(card1: Card, card2: Card, card3: Card, game: Variant) -> u32 {
    match game {
        Variant::Null | Variant::NullOuvert | Variant::NullOuvertHand | Variant::NullHand => {

        },
        Variant::Grand => {

        },
        Variant::Spades | Variant::Hearts | Variant::Diamonds | Variant::Clubs => {

        }
    }

    0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_who_won() {

    }
}


