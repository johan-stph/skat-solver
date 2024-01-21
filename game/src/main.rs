
use std::fs::File;
use std::io::Write;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use skat_solver::solver::bitboard::{BitCard, BitCards};
use skat_solver::solver::{GlobalState, Player, Variant};
use skat_solver::solver::synchronus::ab_tt::DefaultSolver;
use skat_solver::solver::synchronus::local_state::LState;

fn main() {
    create_n_cards(10, 8);
}

fn create_cards(cards: &[u32]) -> BitCards {
    let mut empty = BitCards(0);
    for card in cards {
        empty = empty | BitCard(2_u32.pow(*card))
    }
    empty
}


fn create_n_cards(n: usize, amount: usize) {
    let mut rng = thread_rng();
    let mut file = File::create("eight_cards.txt").unwrap();
    for _ in 0..n {
        for variant in [Variant::Grand, Variant::Clubs, Variant::Spades, Variant::Hearts, Variant::Diamonds] {
            for current_player in [Player::One, Player::Two, Player::Three] {
                let mut numbers: Vec<u32> = (0..32).collect();
                numbers.shuffle(&mut rng);
                let (first, rest) = numbers.split_at(amount);
                let (second, rest) = rest.split_at(amount);
                let (third, _) = rest.split_at(amount);
                let p1 = create_cards(first).0;
                let p2 = create_cards(second).0;
                let p3 = create_cards(third).0;
                let all = p1 | p2 | p3;

                let local_state = LState::new(BitCards(p1 | p2 | p3), current_player);
                let global_state = GlobalState::new((BitCards(p1), BitCards(p2), BitCards(p3)), BitCards(0), Player::One, variant);
                let mut solver = DefaultSolver {
                    global_state,
                    look_up_table: Default::default(),
                };
                let result = solver.solve(local_state);
                let current_player = current_player as u8;
                let variant = variant as u8;

                assert_eq!(all.count_ones(), 3 * amount as u32);
                assert_eq!(p1 & p2, 0);
                assert_eq!(p1 & p3, 0);
                assert_eq!(p2 & p3, 0);
                let to_bet_printed = format!("{p1},{p2},{p3},0,{current_player},{variant},{result}\n");
                file.write_all(to_bet_printed.as_bytes()).expect("");
            }
        }
    }
}