use std::cmp::{max, min};
use crate::bitboard::{BitCard, BitCards, Variant};

#[derive(PartialEq)]
enum Player {
    One,
    Two,
    Three,
}

impl Player {
    fn get_next_player(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::Three,
            Player::Three => Player::One,
        }
    }
}

struct GlobalState {
    player_cards: (BitCards, BitCards, BitCards),
    skat: BitCards,
    alone_player: Player,
    variant: Variant,
}



struct LocalState {
    remaining_cards: BitCards,
    current_played_cards: (BitCard, BitCard),
    current_player: Player,
    current_suit: Option<BitCards>,
    current_points_alone: u32,
}

impl LocalState {
    fn is_terminal(&self) -> bool {
        self.remaining_cards.0 == 0
    }
    fn get_points(&self, global_state: &GlobalState) -> u32 {
        self.current_points_alone + global_state.get_skat_points()
    }
    fn is_max_node(&self, global_state: &GlobalState) -> bool {
        self.current_player == global_state.alone_player
    }
    fn get_next_states(&self, global_state: &GlobalState) -> Vec<LocalState> {

        let mut next_states : Vec<LocalState> = Vec::with_capacity(10);
        //get available cards
        let available: BitCards = match self.current_player {
            Player::One => {
                global_state.player_cards.0 & self.remaining_cards
            },
            Player::Two => {
                global_state.player_cards.1 & self.remaining_cards
            },
            Player::Three => {
                global_state.player_cards.2 & self.remaining_cards
            }
        };
        //if played cards is empty, get all next states with the move played
        let possible_moves: BitCards = calculate_next_moves(&available, self.current_suit);
        for next_move in possible_moves {
            //NO CARD WAS PLAYED
            if self.current_suit.is_none() {
                next_states.push(
                    LocalState {
                        remaining_cards: BitCards(self.remaining_cards.0 & (!next_move.0)),
                        current_played_cards: (next_move, BitCard(0)),
                        current_player: self.current_player.get_next_player(),
                        current_suit: Some(calculate_current_suit_mask(next_move, &global_state.variant)),
                        current_points_alone: self.current_points_alone,
                    }
                );
                continue;
            }
            //TWO CARDS WERE PLAYED
            if self.current_played_cards.1.0 != 0 {
                let winner_card = calculate_who_won(self.current_played_cards, &next_move, &global_state.variant);
                //if winner_card is alone_player add points
                let winner_player = calculate_winner(winner_card.0, global_state);
                next_states.push(
                    LocalState {
                        remaining_cards: BitCards(self.remaining_cards.0 & (!next_move.0)),
                        current_played_cards: (BitCard(0), BitCard(0)),
                        current_player: winner_player,
                        current_suit: None,
                        current_points_alone: self.current_points_alone + winner_card.1,
                    }
                );
                continue;
            }

            // ONE CARD WAS PLAYED

            next_states.push(
                LocalState {
                    remaining_cards: BitCards(self.remaining_cards.0 & (!next_move.0)),
                    current_played_cards: (self.current_played_cards.0, next_move),
                    current_player: self.current_player.get_next_player(),
                    current_suit: self.current_suit,
                    current_points_alone: self.current_points_alone,
                }
            )

        }
        next_states


    }
}

fn calculate_current_suit_mask(first_card: BitCard, variant: &Variant) -> BitCards {
    if first_card.0 & variant.get_binary_mask() != 0 {
        return BitCards(variant.get_binary_mask());
    }
    BitCards(first_card.get_color_mask())
}

fn calculate_winner(winning_card: BitCard, global_state: &GlobalState) -> Player {
    if winning_card.0 & global_state.player_cards.0.0 != 0 {
        Player::One
    }
    else if winning_card.0 & global_state.player_cards.1.0 != 0 {
        Player::Two
    }
    else {
        Player::Three
    }
}

fn calculate_who_won(current_played_cards: (BitCard, BitCard), last_card: &BitCard, variant: &Variant) -> (BitCard, u32) {
    let winning_card = if current_played_cards.0.greater_than(&current_played_cards.1, variant) {
        if current_played_cards.0.greater_than(last_card, variant) {
            current_played_cards.0
        } else {
            *last_card
        }
    }
    else if current_played_cards.1.greater_than(last_card, variant) {
        current_played_cards.1
    } else {
        *last_card
    };
    (winning_card, current_played_cards.0.get_point() + current_played_cards.1.get_point() + last_card.get_point())
}




fn calculate_next_moves(current_cards: &BitCards, suit_mask: Option<BitCards>) -> BitCards {
    if suit_mask.is_none() {
        return *current_cards;
    }
    //now the option must exist
    let suit_mask = suit_mask.unwrap();
    let available = suit_mask & *current_cards;
    if available.0 != 0 {
        return available;
    }
    *current_cards
}


fn minimax(local_state: LocalState, global_state: &GlobalState) -> i32 {
    if local_state.is_terminal() {
        return local_state.get_points(global_state) as i32;
    }
    let mut result: i32;
    if local_state.is_max_node(global_state) {
        result = i32::MIN;
    } else {
        result = i32::MAX;
    }
    for next_states in local_state.get_next_states(global_state) {
        let next_result = minimax(next_states, global_state);
        if local_state.is_max_node(global_state) {
            result = max(result, next_result);
        } else {
            result = min(result, next_result);
        }
    }
    result
}


impl GlobalState {

    fn get_skat_points(&self) -> u32 {
           let mut result = 0;
            for card in self.skat {
                result += card.get_point();
            }
            result
    }


}


#[cfg(test)]
mod tests {
    use crate::bitboard::{BitCard, BitCards, Variant};
    use crate::solver::{GlobalState, LocalState, minimax, Player};

    #[test]
    fn minimax_simple() {
        let global_state = GlobalState {
            player_cards: (BitCards(0b01000100001001011000000011010100), BitCards(0b10010000110000100100001000001011), BitCards(0b00101011000110000001110100000000)),
            skat: BitCards(0b00000000000000000010000000100000),
            alone_player: Player::Three,
            variant: Variant::Grand,
        };
        let local_state = LocalState {
            remaining_cards: BitCards(u32::MAX),
            current_played_cards: (BitCard(0), BitCard(0)),
            current_player: Player::One,
            current_suit: None,
            current_points_alone: 0,
        };
        let result = minimax(local_state, &global_state);
        dbg!(result);
    }

    #[test]
    fn setup() {
        let first = BitCards(0b01000100001001011000000011010100);
        let second = BitCards(0b10010000110000100100001000001011);
        let third = BitCards(0b00101011000110000001110100000000);
        let skat = BitCards(0b00000000000000000010000000100000);
        let all = BitCards(0b11111111111111111111111111111111);
        let new = BitCards(first.0 | second.0 | third.0 | skat.0);
        assert_eq!(first.0 & second.0 & third.0 & skat.0, 0);
        assert_eq!(new.0, all.0);
    }


}