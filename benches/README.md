## Benchmarks

Benchmarks are run with [`criterion.rs`](https://github.com/bheisler/criterion.rs). The reported values are the means and standard deviations. To run the benchmarks yourself, clone the repository and run

```
cargo bench
```

|              | alea                    | fastrand                | rand                    | rand_pcg                |
| ------------ | ----------------------- | ----------------------- | ----------------------- | ----------------------- |
| f64          | 2.0002 ns +/- 21.718 ps | 3.6868 ns +/- 40.030 ps | 4.7417 ns +/- 75.082 ps | 1.4531 ns +/- 73.249 ps |
| f32          | 2.0911 ns +/- 87.985 ps | 2.6735 ns +/- 35.334 ps | 1.8259 ns +/- 34.666 ps | 1.4482 ns +/- 29.133 ps |
| u64          | 2.0062 ns +/- 13.639 ps | 5.1602 ns +/- 132.48 ps | 4.6494 ns +/- 68.218 ps | 1.3561 ns +/- 7.0908 ps |
| u32          | 1.9777 ns +/- 4.3646 ps | 4.5852 ns +/- 120.84 ps | 1.6488 ns +/- 55.921 ps | 1.3668 ns +/- 10.669 ps |
| u64 in range | 4.5298 ns +/- 99.716 ps | 5.1845 ns +/- 240.30 ps | 9.8593 ns +/- 41.581 ps | 5.4817 ns +/- 21.622 ps |
| u32 in range | 2.6764 ns +/- 32.698 ps | 4.3292 ns +/- 20.041 ps | 5.5542 ns +/- 45.249 ps | 4.9378 ns +/- 96.550 ps |
| i64          | 2.0042 ns +/- 23.321 ps | 3.7119 ns +/- 44.196 ps | 4.6275 ns +/- 63.446 ps | 1.3506 ns +/- 17.385 ps |
| i32          | 1.9826 ns +/- 10.443 ps | 2.6396 ns +/- 7.6008 ps | 1.6326 ns +/- 26.437 ps | 1.3668 ns +/- 7.4342 ps |
| i64 in range | 4.4474 ns +/- 96.470 ps | 3.8003 ns +/- 16.704 ps | 5.6069 ns +/- 54.102 ps | 5.5278 ns +/- 84.057 ps |
| i32 in range | 2.4168 ns +/- 19.926 ps | 2.7006 ns +/- 6.5716 ps | 2.8754 ns +/- 26.128 ps | 4.9530 ns +/- 179.12 ps |
