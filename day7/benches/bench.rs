use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day7::{order_hands, order_hands_joker};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day7a", |b| b.iter(|| order_hands(black_box(input))));
    c.bench_function("day7b", |b| b.iter(|| order_hands_joker(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
