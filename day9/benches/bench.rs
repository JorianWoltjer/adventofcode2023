use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day9::{sum_next_predictions, sum_prev_predictions};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day9a", |b| b.iter(|| sum_next_predictions(black_box(input))));
    c.bench_function("day9b", |b| b.iter(|| sum_prev_predictions(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
