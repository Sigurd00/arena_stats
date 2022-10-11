use arena_stats::{self, run};
use criterion::{criterion_group, criterion_main, Criterion};
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("full program", |b| b.iter(|| run::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
