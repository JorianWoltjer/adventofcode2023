use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::{minimum_converted_location, minimum_location_range};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("day5a", |b| {
        b.iter(|| minimum_converted_location(black_box(input)))
    });
    c.bench_function("day5b", |b| {
        b.iter(|| minimum_location_range(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
