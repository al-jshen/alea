# alea

[![Crates.io](https://img.shields.io/crates/v/alea.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/alea)
[![Documentation](https://img.shields.io/badge/docs.rs-alea-5E81AC?style=for-the-badge&labelColor=555555&logoColor=white)](https://docs.rs/alea)
![License](https://img.shields.io/crates/l/alea?label=License&style=for-the-badge&color=62a69b)

A zero-dependency crate for fast number generation, with a focus on ease of use (no more passing `&mut rng` everywhere!).

The implementation is based on [wyrand](https://github.com/wangyi-fudan/wyhash), a high-quality and fast generator.

This crate is heavily inspired by [fastrand](https://github.com/smol-rs/fastrand). For some benchmarks, see [benches](benches).

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
