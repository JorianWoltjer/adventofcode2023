use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::{count_valid, minimum_playable};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day2a", |b| b.iter(|| count_valid(black_box(input))));
    c.bench_function("day2b", |b| b.iter(|| minimum_playable(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
