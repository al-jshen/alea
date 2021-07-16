use std::{
    cell::Cell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread,
    time::Instant,
};

#[derive(Debug)]
/// Random number generator.
pub struct Rng(Cell<u64>);

impl Rng {
    #[inline]
    pub fn new() -> Self {
        Self::with_seed({
            let mut hasher = DefaultHasher::new();
            Instant::now().hash(&mut hasher);
            thread::current().id().hash(&mut hasher);
            let hash = hasher.finish();
            (hash << 1) | 1
        })
    }

    #[inline]
    pub const fn with_seed(seed: u64) -> Self {
        Self { 0: Cell::new(seed) }
    }

    #[inline]
    pub fn get_seed(&self) -> u64 {
        self.0.get()
    }

    #[inline]
    pub fn set_seed(&self, seed: u64) {
        self.0.set(seed);
    }

    #[inline]
    pub fn u64(&self) -> u64 {
        self.0.set(self.0.get() + 0xa0761d6478bd642f);
        let s = self.0.get();
        let t = u128::from(s) * (u128::from(s ^ 0xe7037ed1a0b428db));
        ((t >> 64) as u64) ^ (t as u64)
    }

    #[inline]
    pub fn u32(&self) -> u32 {
        self.u64() as u32
    }

    #[inline]
    pub fn f64(&self) -> f64 {
        (self.u64() as f64) / (u64::MAX as f64)
    }

    #[inline]
    pub fn f32(&self) -> f32 {
        (self.u32() as f32) / (u32::MAX as f32)
    }

    #[inline]
    pub fn i64(&self) -> i64 {
        self.u64() as i64
    }

    #[inline]
    pub fn i32(&self) -> i32 {
        self.u32() as i32
    }

    #[inline]
    pub fn u64_less_than(&self, max: u64) -> u64 {
        let mut r = self.u64();
        let mut hi = mul_high_u64(r, max);
        let mut lo = r.wrapping_mul(max);
        if lo < max {
            let t = max.wrapping_neg() % max;
            while lo < t {
                r = self.u64();
                hi = mul_high_u64(r, max);
                lo = r.wrapping_mul(max);
            }
        }
        hi
    }

    #[inline]
    pub fn u32_less_than(&self, max: u32) -> u32 {
        let mut r = self.u32();
        let mut hi = mul_high_u32(r, max);
        let mut lo = r.wrapping_mul(max);
        if lo < max {
            let t = max.wrapping_neg() % max;
            while lo < t {
                r = self.u32();
                hi = mul_high_u32(r, max);
                lo = r.wrapping_mul(max);
            }
        }
        hi
    }

    #[inline]
    pub fn f64_less_than(&self, max: f64) -> f64 {
        assert!(max > 0., "max must be positive");
        self.f64() * max
    }

    #[inline]
    pub fn f32_less_than(&self, max: f32) -> f32 {
        assert!(max > 0., "max must be positive");
        self.f32() * max
    }

    #[inline]
    pub fn i64_less_than(&self, max: i64) -> i64 {
        self.u64_less_than(max as u64) as i64
    }

    #[inline]
    pub fn i32_less_than(&self, max: i32) -> i32 {
        self.u32_less_than(max as u32) as i32
    }

    #[inline]
    pub fn u64_in_range(&self, min: u64, max: u64) -> u64 {
        assert!(max > min, "max must be greater than min");
        min + self.u64_less_than(max + 1 - min)
    }

    #[inline]
    pub fn u32_in_range(&self, min: u32, max: u32) -> u32 {
        assert!(max > min, "max must be greater than min");
        min + self.u32_less_than(max + 1 - min)
    }

    #[inline]
    pub fn f64_in_range(&self, min: f64, max: f64) -> f64 {
        assert!(max > min, "max must be greater than min");
        min + self.f64_less_than(max - min)
    }

    #[inline]
    pub fn f32_in_range(&self, min: f32, max: f32) -> f32 {
        assert!(max > min, "max must be greater than min");
        min + self.f32_less_than(max - min)
    }

    #[inline]
    pub fn i64_in_range(&self, min: i64, max: i64) -> i64 {
        assert!(max > min, "max must be greater than min");
        min + self.i64_less_than(max + 1 - min)
    }

    #[inline]
    pub fn i32_in_range(&self, min: i32, max: i32) -> i32 {
        assert!(max > min, "max must be greater than min");
        min + self.i32_less_than(max + 1 - min)
    }

    #[inline]
    pub fn bool(&self) -> bool {
        self.f32() < 0.5
    }

    #[inline]
    pub fn wyhash_u64(&self) -> u64 {
        self.0.set(self.0.get() + 0x60bee2bee120fc15);
        let mut tmp: u128 = (self.0.get() as u128) * 0xa3b195354a39b70d;
        let m1: u64 = ((tmp >> 64) ^ tmp) as u64;
        tmp = (m1 as u128) * 0x1b03738712fad5c9;
        ((tmp >> 64) ^ tmp) as u64
    }

    #[inline]
    pub fn wyhash_f64(&self) -> f64 {
        (self.wyhash_u64() as f64) / (u64::MAX as f64)
    }
}

#[inline]
fn mul_high_u32(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}

#[inline]
fn mul_high_u64(a: u64, b: u64) -> u64 {
    (((a as u128) * (b as u128)) >> 64) as u64
}

thread_local! {
    static RNG: Rng = Rng::new();
}

#[doc = "Get the seed for the random number generator."]
pub fn get_seed() -> u64 {
    RNG.with(|rng| rng.get_seed())
}

#[doc = "Set the seed for the random number generator."]
pub fn set_seed(seed: u64) {
    RNG.with(|rng| rng.set_seed(seed));
}

macro_rules! impl_rng_functions {
($doc1: tt $doc2: tt | $($fn: ident $type: ident $($arg: ident)* ),+ $(,)?) => {
    $(
    #[doc = $doc1]
    #[doc = stringify!($type)]
    #[doc = $doc2]
    pub fn $fn( $($arg: $type, )* ) -> $type {
        RNG.with(|rng| rng.$fn( $($arg, )* ))
    }
    )+
};
}

macro_rules! impl_rng_functions_helper_1 {
($doc1: tt $doc2: tt | $($type: ident, )+) => {
    impl_rng_functions!($doc1 $doc2 | $($type $type, )+);
};
}

macro_rules! impl_rng_functions_helper_2 {
($doc1: tt $doc2: tt | $($fn: tt $type: ident, )+) => {
    impl_rng_functions!($doc1 $doc2 | $($fn $type max, )+);
};
}

macro_rules! impl_rng_functions_helper_3 {
($($fn: tt $type: ident, )+) => {
    impl_rng_functions!("Generate a random `" "` value in the range [min, max] (i.e., both endpoints are included)." | $($fn $type min max, )+);
}
}

impl_rng_functions_helper_1!("Generate a random `" "` value." | u64, u32, i64, i32, bool,);
impl_rng_functions_helper_1!("Generate a random `" "` value in the range [0, 1)." | f64, f32,);
impl_rng_functions_helper_2!("Generate a random `" "` value less than `max`." | u64_less_than u64, u32_less_than u32, i64_less_than i64, i32_less_than i32,);
impl_rng_functions_helper_2!("Generate a random `" "` value in the range [0, max)." | f64_less_than f64, f32_less_than f32,);
impl_rng_functions_helper_3!(u64_in_range u64, u32_in_range u32, f64_in_range f64, f32_in_range f32, i64_in_range i64, i32_in_range i32,);
