# simd-slice

This crate provides a wrapper type `struct SimdSlice(&[T])` with simd-enabled operations.

Relying on auto-vectorization by compiler might fail for various reasons.
Use this crate to explicitly enable the vectorization without boilerplate.

## Example

```rs
let a: Vec<i32> = {
    let mut a = vec![0_i32; N];
    for i in 0..N {
        a[i] = (i % 12345) as i32;
    }
    a
};

use simd_slice::AsSimdSlice;

// unaligned, odd-length cases are supported
let sum: i32 = a.as_simd_slice().sum(); 
let min: Option<i32> = a.as_simd_slice().min();
let max: Option<i32> = a.as_simd_slice().max();
```

## Benchmark

reduce-sum on a `Vec<i32>` with 1000000 items

```
naive:   251.136µs (R²=1.000, 4039 iterations in 62 samples)
rayon:   108.663µs (R²=0.997, 9542 iterations in 71 samples)
simd :    52.341µs (R²=0.998, 20470 iterations in 79 samples)
both1:   148.563µs (R²=0.999, 7165 iterations in 68 samples)
both2:    61.943µs (R²=0.992, 16915 iterations in 77 samples)
both3:    82.678µs (R²=0.997, 12705 iterations in 74 samples)
```

- simd  = simd-slice
- both1 = rayon chunks(4) + std::simd
- both2 = rayon chunks(4096) + std::simd
- both3 = rayon chunks(slice.len()/num_cores) + std::simd
