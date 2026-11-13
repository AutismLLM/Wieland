---
id: opt-simd-portable
title: Reach for autovectorization or portable SIMD before hand-written intrinsics
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, simd, vectorization, portability]
applies_to:
  - "numeric kernels over slices that could vectorize"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-simd-portable
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For vectorizable numeric kernels, first help LLVM autovectorize with clean iterator loops, then use portable SIMD (`std::simd` on nightly, or the `wide` crate on stable) before dropping to architecture-specific intrinsics.

## Why
SIMD processes 4, 8, or more values per instruction, but hand-written platform intrinsics are unsafe, non-portable, and easy to get wrong. Simple iterator-and-`chunks_exact` loops often autovectorize for free on stable, and portable SIMD gives explicit control while staying cross-architecture. Falling straight to `_mm256_*` intrinsics trades safety and portability for control you rarely need.

## Do
```rust
fn add_arrays(a: &[f32], b: &[f32], out: &mut [f32]) {
    for ((x, y), o) in a.iter().zip(b).zip(out.iter_mut()) {
        *o = x + y;
    }
}
```

## Don't
```rust
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn add_arrays_avx2(a: &[f32], b: &[f32], out: &mut [f32]) {
    use std::arch::x86_64::*;
    for i in (0..a.len()).step_by(8) {
        let va = _mm256_loadu_ps(a.as_ptr().add(i));
        let vb = _mm256_loadu_ps(b.as_ptr().add(i));
        _mm256_storeu_ps(out.as_mut_ptr().add(i), _mm256_add_ps(va, vb));
    }
}
```

## Notes
Autovectorization is fragile — early exits and indexing can defeat it, so verify hot kernels with `cargo asm`. Reserve raw intrinsics for the rare case where portable approaches leave measurable performance on the table, and gate them behind runtime feature detection.
