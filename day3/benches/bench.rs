use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3::{sum_engine_parts, sum_gears};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day3a", |b| b.iter(|| sum_engine_parts(black_box(input))));
    c.bench_function("day3b", |b| b.iter(|| sum_gears(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
