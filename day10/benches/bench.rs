use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day10::{furthest_in_loop, count_squares_inside};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day10a", |b| b.iter(|| furthest_in_loop(black_box(input))));
    c.bench_function("day10b", |b| b.iter(|| count_squares_inside(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
