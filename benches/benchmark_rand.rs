use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::uniform::Uniform;
use rand::prelude::*;

fn criterion_u64(c: &mut Criterion) {
    c.bench_function("fastrand u64", |b| {
        b.iter(|| fastrand::u64(u64::MIN..u64::MAX))
    });
    c.bench_function("alea u64", |b| b.iter(|| alea::u64()));
    c.bench_function("rand u64", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen::<u64>());
    });
    c.bench_function("rand_pcg u64", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen::<u64>());
    });
    c.bench_function("fastrand u64 in range", |b| {
        b.iter(|| fastrand::u64(20..200))
    });
    c.bench_function("alea u64 in range", |b| {
        b.iter(|| alea::u64_in_range(20, 200))
    });
    c.bench_function("rand u64 in range", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen_range(20_u64..200));
    });
    c.bench_function("rand u64 in range with uniform", |b| {
        let mut rng = thread_rng();
        let unif = Uniform::new(20_u64, 200);
        b.iter(|| unif.sample(&mut rng));
    });
    c.bench_function("rand_pcg u64 in range", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen_range(20_u64..200));
    });
}

fn criterion_u32(c: &mut Criterion) {
    c.bench_function("fastrand u32", |b| {
        b.iter(|| fastrand::u32(u32::MIN..u32::MAX))
    });
    c.bench_function("alea u32", |b| b.iter(|| alea::u32()));
    c.bench_function("rand u32", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen::<u32>());
    });
    c.bench_function("rand_pcg u32", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen::<u32>());
    });
    c.bench_function("fastrand u32 in range", |b| {
        b.iter(|| fastrand::u32(20..200))
    });
    c.bench_function("alea u32 in range", |b| {
        b.iter(|| alea::u32_in_range(20, 200))
    });
    c.bench_function("rand u32 in range", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen_range(20_u32..200));
    });
    c.bench_function("rand u32 in range with uniform", |b| {
        let mut rng = thread_rng();
        let unif = Uniform::new(20_u32, 200);
        b.iter(|| unif.sample(&mut rng));
    });
    c.bench_function("rand_pcg u32 in range", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen_range(20_u32..200));
    });
}

fn criterion_i32(c: &mut Criterion) {
    c.bench_function("fastrand i32", |b| {
        b.iter(|| fastrand::i32(i32::MIN..i32::MAX))
    });
    c.bench_function("alea i32", |b| b.iter(|| alea::i32()));
    c.bench_function("rand i32", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen::<i32>());
    });
    c.bench_function("rand_pcg i32", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen::<i32>());
    });

    c.bench_function("fastrand i32 in range", |b| {
        b.iter(|| fastrand::i32(-20..10))
    });
    c.bench_function("alea i32 in range", |b| {
        b.iter(|| alea::i32_in_range(-20, 10))
    });
    c.bench_function("rand i32 in range", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen_range(-20_i32..10));
    });
    c.bench_function("rand i32 in range with uniform", |b| {
        let mut rng = thread_rng();
        let unif = Uniform::new(20_i32, 200);
        b.iter(|| unif.sample(&mut rng));
    });
    c.bench_function("rand_pcg i32 in range", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen_range(20_i32..200));
    });
}

fn criterion_i64(c: &mut Criterion) {
    c.bench_function("fastrand i64", |b| {
        b.iter(|| fastrand::i64(i64::MIN..i64::MAX))
    });
    c.bench_function("alea i64", |b| b.iter(|| alea::i64()));
    c.bench_function("rand i64", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen::<i64>());
    });
    c.bench_function("rand_pcg i64", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen::<i64>());
    });
    c.bench_function("fastrand i64 in range", |b| {
        b.iter(|| fastrand::i64(-20..10))
    });
    c.bench_function("alea i64 in range", |b| {
        b.iter(|| alea::i64_in_range(-20, 10))
    });
    c.bench_function("rand i64 in range", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen_range(-20_i64..10));
    });
    c.bench_function("rand i64 in range with uniform", |b| {
        let mut rng = thread_rng();
        let unif = Uniform::new(20_i64, 200);
        b.iter(|| unif.sample(&mut rng));
    });
    c.bench_function("rand_pcg i64 in range", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen_range(20_i64..200));
    });
}

fn criterion_f64(c: &mut Criterion) {
    c.bench_function("fastrand f64", |b| b.iter(|| fastrand::f64()));
    c.bench_function("alea f64", |b| b.iter(|| alea::f64()));
    c.bench_function("rand f64", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen::<f64>());
    });
    c.bench_function("rand_pcg f32", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen::<f64>());
    });
}

fn criterion_f32(c: &mut Criterion) {
    c.bench_function("fastrand f32", |b| b.iter(|| fastrand::f32()));
    c.bench_function("alea f32", |b| b.iter(|| alea::f32()));
    c.bench_function("rand f32", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| rng.gen::<f32>());
    });
    c.bench_function("rand_pcg f64", |b| {
        let mut rng = rand_pcg::Lcg128Xsl64::from_entropy();
        b.iter(|| rng.gen::<f32>());
    });
}

criterion_group!(
    benches,
    criterion_u32,
    criterion_u64,
    criterion_f64,
    criterion_f32,
    criterion_i64,
    criterion_i32
);
criterion_main!(benches);
