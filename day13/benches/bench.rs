use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day13::{find_symmetries, find_imperfect_symmetries};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day13a", |b| b.iter(|| find_symmetries(black_box(input))));
    c.bench_function("day13b", |b| b.iter(|| find_imperfect_symmetries(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
