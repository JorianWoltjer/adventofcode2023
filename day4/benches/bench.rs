use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4::{count_copies, score_winning_numbers};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day4a", |b| b.iter(|| score_winning_numbers(black_box(input))));
    c.bench_function("day4b", |b| b.iter(|| count_copies(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
