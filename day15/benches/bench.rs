use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15::{calc_hash_boxes, sum_hashes};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day15a", |b| b.iter(|| sum_hashes(black_box(input))));
    c.bench_function("day15b", |b| b.iter(|| calc_hash_boxes(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
