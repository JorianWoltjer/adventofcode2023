use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day6::{multiply_ways_to_win, big_ways_to_win};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day6a", |b| b.iter(|| multiply_ways_to_win(black_box(input))));
    c.bench_function("day6b", |b| b.iter(|| big_ways_to_win(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
