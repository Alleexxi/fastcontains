use criterion::{criterion_group, criterion_main, Criterion};
use fastcontains::fastcontains;

fn string_contains(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

fn fast_contains(haystack: &str, needle: &str) -> bool {
    haystack.fast_contains(needle)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("string contains", |b| {
        b.iter(|| string_contains("The quick brown fox jumps over the lazy dog", "fox"))
    });

    c.bench_function("fast contains", |b| {
        b.iter(|| fast_contains("The quick brown fox jumps over the lazy dog", "fox"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
