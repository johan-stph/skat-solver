
/*
30 + 2 Karten
3 Spieler (Vorhaut, Hinterhand, Mittelhand)



Spielphasen

1. Reizen (Reihenfolge entscheidend)

2. Optionale Skataufnahme

3. Spiel ansagen

4. Spiel ausführen

 */

use rand::seq::SliceRandom;
use crate::{Card, Rank, Suit, Variant};

struct Player {
    points: u32,
    hand: Vec<Card>,
}


impl Variant {
    fn value(&self) -> u32 {
        match self {
            Variant::Grand => 24,
            Variant::NullHand => 35,
            Variant::NullOuvert => 35,
            Variant::NullOuvertHand => 46,
            Variant::Null => 23,
            Variant::Diamonds => 9,
            Variant::Hearts => 10,
            Variant::Spades => 11,
            Variant::Clubs => 12,
        }
    }

}





pub fn generate_card_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(32);
    for suit in [Suit::Kreuz, Suit::Piqus, Suit::Heart, Suit::Karo].iter() {
        for rank in [Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace].iter() {
            deck.push(Card { suit: *suit, rank: *rank });
        }
    }
    deck.shuffle(&mut rand::thread_rng());
    deck
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_card_deck() {
        let deck = generate_card_deck();
        dbg!(&deck);
        assert_eq!(deck.len(), 32);
    }

    #[test]
    fn generate_player_and_cards() {
        let deck = generate_card_deck();
        let mut hand1 = deck;
        let mut hand2 = hand1.split_off(10);
        let mut hand3 = hand2.split_off(10);
        let skat = hand3.split_off(10);


        let player1 = Player { points: 0, hand: hand1 };
        let player2 = Player { points: 0, hand: hand2 };
        let player3 = Player { points: 0, hand: hand3 };
        assert_eq!(player1.hand.len(), 10);
        assert_eq!(player2.hand.len(), 10);
        assert_eq!(player3.hand.len(), 10);
        assert_eq!(skat.len(), 2);
        dbg!(&skat);

    }
}