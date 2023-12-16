use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day11::sum_expanded_lengths;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day11a", |b| b.iter(|| sum_expanded_lengths(black_box(input), 2)));
    c.bench_function("day11b", |b| b.iter(|| sum_expanded_lengths(black_box(input), 1000000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
