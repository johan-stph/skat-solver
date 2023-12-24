use skat_solver::game::generate_card_deck;





fn main() {
    let mut hand1 = generate_card_deck(1);
    let mut hand2 = hand1.split_off(10);
    let mut hand3 = hand2.split_off(10);
    let skat = hand3.split_off(10);

    dbg!(&hand1);
    dbg!(&hand2);
    dbg!(&hand3);
    dbg!(&skat);
    
}
