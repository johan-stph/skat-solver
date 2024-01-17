use std::fs;
use criterion::{Criterion, criterion_group, criterion_main, SamplingMode};
use skat_solver::solver::bitboard::BitCards;
use skat_solver::solver::{GlobalState, Player, Variant};
use skat_solver::solver::synchronus::ab_tt::Solver;
use skat_solver::solver::synchronus::local_state::LState;


fn run_test(line: &str) -> Result<(), (u8, u8)> {
    let data: Vec<&str> = line.split(',').collect();
    let p1 = BitCards(data[0].parse::<u32>().unwrap());
    let p2 = BitCards(data[1].parse::<u32>().unwrap());
    let p3 = BitCards(data[2].parse::<u32>().unwrap());
    let skat= BitCards(data[3].parse::<u32>().unwrap());
    let current_player: Player = Player::from(data[4].parse::<u8>().unwrap());
    let variant: Variant = Variant::from(data[5].parse::<u8>().unwrap());
    let score = data[6].parse::<u8>().unwrap();
    let local_state = LState::new(p1 | p2 | p3, current_player);
    let global_state = GlobalState::new((p1, p2, p3), skat, Player::One, variant);
    let mut solver = Solver {
        global_state,
        look_up_table: Default::default(),
    };
    let result = solver.ab_tt(local_state, 0, 120);
    assert!((0..=120).contains(&result));
    let result = result as u8;
    if result == score {
        return Ok(());
    }
    Err((result, score))
}



fn ab_tt_normal_seven_cards() {
    let input = fs::read_to_string("data/seven_cards.txt").unwrap();
    let len = input.lines().count();
    let mut successes = 0;

    for line in input.lines() {
        let result = run_test(line);
        if let Ok(()) = result { successes +=1 }
    }
    assert_eq!(successes, len);
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let mut c = Criterion::default().sample_size(10);
    let mut group = c.benchmark_group("ab_tt");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("7 moves", |b| b.iter(ab_tt_normal_seven_cards));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);