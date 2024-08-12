use criterion::{criterion_group, criterion_main, Criterion};
use splitter_wasm::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Marwa Original", |b| {
        b.iter(|| splitter1(get_inputs().0, get_inputs().1))
    });
    c.bench_function("Claude AI", |b| {
        b.iter(|| splitter2(get_inputs().0, &get_inputs().1))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);