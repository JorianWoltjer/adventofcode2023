use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day8::{count_steps, count_steps_all_paths};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day8a", |b| b.iter(|| count_steps(black_box(input))));
    c.bench_function("day8b", |b| b.iter(|| count_steps_all_paths(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
