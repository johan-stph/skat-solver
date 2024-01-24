use std::fs;
use criterion::{Criterion, criterion_group, criterion_main};
use skat_solver::solver::bitboard::BitCards;
use skat_solver::solver::{Player, Variant};
use skat_solver::solver::bitstates::{BitGlobal, BitLocal};
use skat_solver::solver::concurrent::lazy_smp::LazyMoreEnhancedSolver;


fn run_test(line: &str) -> Result<(), (u8, u8)> {
    let data: Vec<&str> = line.split(',').collect();
    let p1 = BitCards(data[0].parse::<u32>().unwrap());
    let p2 = BitCards(data[1].parse::<u32>().unwrap());
    let p3 = BitCards(data[2].parse::<u32>().unwrap());
    let current_player: Player = Player::from(data[4].parse::<u8>().unwrap());
    let variant: Variant = Variant::from(data[5].parse::<u8>().unwrap());
    let score = data[6].parse::<u8>().unwrap();
    let global_state = BitGlobal::new(p1, p2, p3, variant);
    let local_state = BitLocal::new((p1 | p2 | p3).0, current_player, global_state.skat);
    let mut solver = LazyMoreEnhancedSolver::new(global_state);
    let result = solver.solve(local_state);
    assert!((0..=120).contains(&result));
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

fn ab_tt_enhanced_ten_cards() {
    let input = fs::read_to_string("data/full_game.txt").unwrap();
    let len = input.lines().count();
    let mut successes = 0;

    for line in input.lines() {
        let result = run_test(line);
        if let Ok(()) = result { successes +=1 }
    }
    assert_eq!(successes, len);
}





pub fn criterion_benchmark(_: &mut Criterion) {
    let mut c = Criterion::default().sample_size(10);
    let mut group = c.benchmark_group("ab_tt");
    //group.sampling_mode(SamplingMode::Flat);
    group.bench_function("7 moves", |b| b.iter(ab_tt_normal_seven_cards));
    //group.bench_function("10 moves enhanced", |b| b.iter(ab_tt_enhanced_ten_cards));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);