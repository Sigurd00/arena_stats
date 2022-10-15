use arena_stats::{analyser, parser};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let games = parser::parse_games(String::from("testdata.csv").into()).unwrap();
    c.bench_function("parser", |b| {
        b.iter(|| parser::parse_games(String::from("testdata.csv").into()))
    });
    c.bench_with_input(BenchmarkId::new("analyser", "games"), &games, |b, s| {
        b.iter(|| analyser::start(s))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
