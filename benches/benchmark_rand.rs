use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_u64(c: &mut Criterion) {
    c.bench_function("fastrand u64", |b| {
        b.iter(|| fastrand::u64(u64::MIN..u64::MAX))
    });
    c.bench_function("alea u64", |b| b.iter(|| alea::u64()));
    c.bench_function("alea wyhash u64", |b| b.iter(|| alea::wyhash_u64()));
}

fn criterion_f64(c: &mut Criterion) {
    c.bench_function("fastrand f64", |b| b.iter(|| fastrand::f64()));
    c.bench_function("alea f64", |b| b.iter(|| alea::f64()));
    c.bench_function("alea wyhash f64", |b| b.iter(|| alea::wyhash_f64()));
}

criterion_group!(benches, criterion_u64, criterion_f64);
criterion_main!(benches);
