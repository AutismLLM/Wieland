---
id: test-criterion-bench
title: Benchmark with criterion instead of ad-hoc Instant timing
category: testing
severity: info
tier: 1
enforceability: judge
tags: [benchmarking, performance, tooling]
applies_to:
  - "benches/ files"
  - "hand-rolled Instant::now timing loops"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-criterion-bench
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `criterion` for benchmarks, and wrap inputs and results in `black_box` so the optimizer cannot elide the work being measured.

## Why
Criterion handles warmup, runs many iterations, detects outliers, and compares against saved baselines, giving statistically meaningful numbers. A single `Instant::now()` timing is noisy and easily gamed by the optimizer, which can delete a computation whose result is never observed — `black_box` is what forces the work to actually happen.

## Do
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

## Don't
```rust
fn main() {
    let start = std::time::Instant::now();
    fibonacci(20);
    println!("took {:?}", start.elapsed());
}
```
