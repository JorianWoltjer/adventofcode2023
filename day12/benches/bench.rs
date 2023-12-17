use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day12::{part1_function, part2_function};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day12a", |b| b.iter(|| part1_function(black_box(input))));
    c.bench_function("day12b", |b| b.iter(|| part2_function(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
