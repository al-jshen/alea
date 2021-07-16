use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::thread;
use std::time::Instant;

#[derive(Debug)]
pub struct Rng(Cell<u64>);

impl Rng {
    pub fn new() -> Self {
        Self::with_seed({
            let mut hasher = DefaultHasher::new();
            Instant::now().hash(&mut hasher);
            thread::current().id().hash(&mut hasher);
            let hash = hasher.finish();
            (hash << 1) | 1
        })
    }

    pub const fn with_seed(seed: u64) -> Self {
        Self { 0: Cell::new(seed) }
    }

    pub fn get_seed(&self) -> u64 {
        self.0.get()
    }

    pub fn u64(&self) -> u64 {
        self.0.set(self.0.get() + 0xa0761d6478bd642f);
        let s = self.0.get();
        let t = u128::from(s) * u128::from(s ^ 0xe7037ed1a0b428db);
        ((t >> 64) as u64) ^ (t as u64)
    }

    pub fn f64(&self) -> f64 {
        (self.u64() as f64) / (u64::MAX as f64)
    }

    pub fn wyhash_u64(&self) -> u64 {
        self.0.set(self.0.get() + 0x60bee2bee120fc15);
        let mut tmp: u128 = (self.0.get() as u128) * 0xa3b195354a39b70d;
        let m1: u64 = ((tmp >> 64) ^ tmp) as u64;
        tmp = (m1 as u128) * 0x1b03738712fad5c9;
        ((tmp >> 64) ^ tmp) as u64
    }

    pub fn wyhash_f64(&self) -> f64 {
        (self.wyhash_u64() as f64) / (u64::MAX as f64)
    }
}

thread_local! {
    static RNG: Rng = Rng::new();
}

macro_rules! impl_rng_functions {
($($fn: ident $type: ident ),+) => {
    $(
    pub fn $fn() -> $type {
        RNG.with(|rng| rng.$fn())
    }
    )+
};
}

impl_rng_functions!(f64 f64, u64 u64);
impl_rng_functions!(wyhash_f64 f64, wyhash_u64 u64);
