---
id: perf-black-box-bench
title: Wrap benchmark inputs and outputs in black_box
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, benchmarking, testing]
applies_to:
  - "criterion bench_function bodies"
  - "microbenchmarks measuring a computation"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-black-box-bench
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In benchmarks, pass inputs through `black_box` and return the result through `black_box` so the optimizer cannot fold or eliminate the code under test.

## Why
The compiler aggressively removes computations whose results are unused and precomputes constant inputs. In a benchmark this means you can measure nothing at all instead of the code you care about. `std::hint::black_box` hides values from the optimizer, forcing the input to be treated as unknown and the result to actually be computed each iteration.

## Do
```rust
use criterion::{black_box, Criterion};

fn benchmark_good(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| {
            let result = expensive_computation(black_box(42));
            black_box(result)
        });
    });
}
```

## Don't
```rust
use criterion::Criterion;

fn benchmark_bad(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| {
            let result = expensive_computation(42);
        });
    });
}
```

## Notes
`black_box` is in std since Rust 1.66 (`std::hint::black_box`); criterion re-exports it. Block both ends: shielding only the output still lets a constant input be folded away.
