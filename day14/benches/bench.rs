use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day14::{count_roll_cycle, count_roll_north};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day14a", |b| b.iter(|| count_roll_north(black_box(input))));
    c.bench_function("day14b", |b| b.iter(|| count_roll_cycle(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
