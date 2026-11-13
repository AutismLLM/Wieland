---
id: perf-chain-avoid
title: Avoid Iterator::chain in hot inner loops
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, iterators, hot-path]
applies_to:
  - "iterator chains inside a tight inner loop"
  - "chain over slices called millions of times"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-chain-avoid
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In performance-critical inner loops, prefer separate loops or a pre-combined collection over `Iterator::chain`.

## Why
`chain` checks which underlying iterator is active on every `.next()` call, adding a branch per element. In a hot loop that branch-prediction overhead is measurable, and chaining several iterators compounds it. Splitting into separate branch-free loops, or building one combined buffer before the loop, removes the per-item cost.

## Do
```rust
fn process_hot_path(a: &[i32], b: &[i32]) -> i64 {
    let mut sum = 0i64;
    for _ in 0..1_000_000 {
        for x in a {
            sum += *x as i64;
        }
        for x in b {
            sum += *x as i64;
        }
    }
    sum
}
```

## Don't
```rust
fn process_hot_path(a: &[i32], b: &[i32]) -> i64 {
    let mut sum = 0i64;
    for _ in 0..1_000_000 {
        for x in a.iter().chain(b.iter()) {
            sum += *x as i64;
        }
    }
    sum
}
```

## Notes
`chain` is perfectly fine for one-time iteration, lazy short-circuiting (`find`), or a handful of elements. This only matters when the chain sits in a measured hot path.
