# alea


[![Crates.io](https://img.shields.io/crates/v/alea)](https://crates.io/crates/alea)
[![Documentation](https://docs.rs/alea/badge.svg)](https://docs.rs/alea)
![License](https://img.shields.io/crates/l/alea?label=License)

A zero-dependency crate for fast number generation, with a focus on ease of use (no more passing `&mut rng` everywhere!).

The implementation is based on [wyrand](https://github.com/wangyi-fudan/wyhash), a high-quality and fast generator. 

This crate is heavily inspired by [`fastrand`](https://github.com/smol-rs/fastrand).

## Usage

Add the following to your `Cargo.toml`:
```rust
[dependencies]
alea = "0.2"
```

## Examples

Flip a coin:
```rust
if alea::bool() {
  println!("heads");
} else {
  println!("tails");
}
```

Generate a `u64`:
```rust
let u = alea::u64();
```

Fill a vector with random integers in some range:
```rust
let n = 1_000_000;

let mut v = vec![0; n];
for i in 0..n {
  v[i] = alea::i32_in_range(-200, 150);
}
```

Seed the generator to get reproducible results:
```rust
alea::set_seed(10);
```

## Benchmarks

Benchmarks are run with [`criterion.rs`](https://github.com/bheisler/criterion.rs). The reported values are the means and standard deviations. To run the benchmarks yourself, clone the repository and run 
```
cargo bench
```

|              | alea                    | fastrand                | rand                    |
|--------------|-------------------------|-------------------------|-------------------------|
| f64          | **2.0002 ns +/- 21.718 ps** | 3.6868 ns +/- 40.030 ps | 4.7417 ns +/- 75.082 ps |
| f32          | **2.0911 ns +/- 87.985 ps** | 2.6735 ns +/- 35.334 ps | 1.8259 ns +/- 34.666 ps |
| u64          | **2.0062 ns +/- 13.639 ps** | 5.1602 ns +/- 132.48 ps | 4.6494 ns +/- 68.218 ps |
| u32          | 1.9777 ns +/- 4.3646 ps | 4.5852 ns +/- 120.84 ps | **1.6488 ns +/- 55.921 ps** |
| u64 in range | **4.5298 ns +/- 99.716 ps** | 5.1845 ns +/- 240.30 ps | 9.8593 ns +/- 41.581 ps |
| u32 in range | **2.6764 ns +/- 32.698 ps** | 4.3292 ns +/- 20.041 ps | 5.5542 ns +/- 45.249 ps |
| i64          | **2.0042 ns +/- 23.321 ps** | 3.7119 ns +/- 44.196 ps | 4.6275 ns +/- 63.446 ps |
| i32          | 1.9826 ns +/- 10.443 ps | 2.6396 ns +/- 7.6008 ps | **1.6326 ns +/- 26.437 ps** |
| i64 in range | 4.4474 ns +/- 96.470 ps | **3.8003 ns +/- 16.704 ps** | 5.6069 ns +/- 54.102 ps |
| i32 in range | **2.4168 ns +/- 19.926 ps** | 2.7006 ns +/- 6.5716 ps | 2.8754 ns +/- 26.128 ps |
