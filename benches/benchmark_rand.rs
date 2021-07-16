use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_u64(c: &mut Criterion) {
    c.bench_function("fastrand u64", |b| {
        b.iter(|| fastrand::u64(u64::MIN..u64::MAX))
    });
    c.bench_function("alea u64", |b| b.iter(|| alea::u64()));
    c.bench_function("fastrand u64 in range", |b| {
        b.iter(|| fastrand::u64(20..200))
    });
    c.bench_function("alea u64 in range", |b| {
        b.iter(|| alea::u64_in_range(20, 200))
    });
    // c.bench_function("alea wyhash u64", |b| b.iter(|| alea::wyhash_u64()));
}

fn criterion_u32(c: &mut Criterion) {
    c.bench_function("fastrand u32", |b| {
        b.iter(|| fastrand::u32(u32::MIN..u32::MAX))
    });
    c.bench_function("alea u32", |b| b.iter(|| alea::u32()));
    c.bench_function("fastrand u32 in range", |b| {
        b.iter(|| fastrand::u32(20..200))
    });
    c.bench_function("alea u32 in range", |b| {
        b.iter(|| alea::u32_in_range(20, 200))
    });
}

fn criterion_i32(c: &mut Criterion) {
    c.bench_function("fastrand i32", |b| {
        b.iter(|| fastrand::i32(i32::MIN..i32::MAX))
    });
    c.bench_function("alea i32", |b| b.iter(|| alea::i32()));
    c.bench_function("fastrand i32 in range", |b| {
        b.iter(|| fastrand::i32(-20..10))
    });
    c.bench_function("alea i32 in range", |b| {
        b.iter(|| alea::i32_in_range(-20, 10))
    });
}

fn criterion_f64(c: &mut Criterion) {
    c.bench_function("fastrand f64", |b| b.iter(|| fastrand::f64()));
    c.bench_function("alea f64", |b| b.iter(|| alea::f64()));
    // c.bench_function("alea wyhash f64", |b| b.iter(|| alea::wyhash_f64()));
}

criterion_group!(benches, criterion_u64);
criterion_main!(benches);
