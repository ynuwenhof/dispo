use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_is_valid(c: &mut Criterion) {
    c.bench_function("is_valid", |b| {
        b.iter(|| dispo::is_valid(black_box("alice@example.com")))
    });
}

fn bench_is_valid_domain(c: &mut Criterion) {
    c.bench_function("is_valid_domain", |b| {
        b.iter(|| dispo::is_valid_domain(black_box("example.com")))
    });
}

criterion_group!(benches, bench_is_valid, bench_is_valid_domain);
criterion_main!(benches);
