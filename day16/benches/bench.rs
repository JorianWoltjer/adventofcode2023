use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day16::{best_beam_energy, count_beam_energy};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day16a", |b| b.iter(|| count_beam_energy(black_box(input))));
    c.bench_function("day16b", |b| b.iter(|| best_beam_energy(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
