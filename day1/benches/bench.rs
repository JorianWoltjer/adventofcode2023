use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::{calibrate, spelled_calibrate};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day1a", |b| b.iter(|| calibrate(black_box(input))));
    c.bench_function("day1b", |b| b.iter(|| spelled_calibrate(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
