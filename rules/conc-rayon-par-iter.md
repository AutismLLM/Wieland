---
id: conc-rayon-par-iter
title: Use rayon's par_iter for CPU-bound data parallelism
category: concurrency
severity: info
tier: 1
enforceability: judge
tags: [rayon, parallelism, cpu-bound]
applies_to:
  - "CPU-bound map/filter/fold/sort over large slices"
  - "single-threaded iterator over a large collection"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#conc-rayon-par-iter
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For CPU-bound work over large collections, switch `.iter()` to rayon's `.par_iter()` and let its work-stealing scheduler use all cores.

## Why
Rayon's API mirrors the standard iterator adapters, so a data-parallel workload often becomes a one-word change that yields near-linear speedup across cores. It balances load, chunks automatically, and composes with the full adapter chain. Reserve it for CPU-bound computation: rayon threads block, so IO-bound concurrency belongs on an async runtime instead.

## Do
```rust
use rayon::prelude::*;

fn sum_squares(data: &[f64]) -> f64 {
    data.par_iter().map(|x| x * x).sum()
}

fn normalize(data: &mut [f64]) {
    let max = data.par_iter().cloned().reduce(|| f64::NEG_INFINITY, f64::max);
    data.par_iter_mut().for_each(|x| *x /= max);
}

fn sort_large(data: &mut [f64]) {
    data.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}
```

## Don't
```rust
fn sum_squares(data: &[f64]) -> f64 {
    data.iter().map(|x| x * x).sum()
}

fn normalize(data: &mut [f64]) {
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    data.iter_mut().for_each(|x| *x /= max);
}
```

## Notes
Small collections or trivial per-element work are often faster sequentially because thread-spawn and coordination overhead dominates — profile before parallelizing. For IO-bound concurrency use async (`tokio`), not rayon. Rayon does not prevent data races on shared mutable state; guard it with atomics or a `Mutex`.
