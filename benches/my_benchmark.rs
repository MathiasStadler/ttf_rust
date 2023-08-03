use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ttf_rust::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
