use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use game_of_life::GameBoard;

pub fn game_of_life_benchmark(c: &mut Criterion) {
    for n in [128, 256, 512, 1024, 2048, 4096] {
        let init_state = Vec::new();
        let mut gameboard = GameBoard::new(n, n, &init_state);
        c.bench_function("GameBoard::next_state", |b| {
            b.iter(|| gameboard.next_state())
        });
    }
}

criterion_group!(benches, game_of_life_benchmark);
criterion_main!(benches);
