---
id: perf-iter-over-index
title: Prefer iterators over manual index-range loops
category: performance
severity: warn
tier: 2
enforceability: gate
tags: [performance, iterators, bounds-check]
applies_to:
  - "for i in 0..slice.len() { slice[i] }"
  - "index-based traversal of one or more slices"
rust_edition: "2024"
msrv: null
detect:
  clippy: [needless_range_loop]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-iter-over-index
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Traverse collections with `iter()`/`iter_mut()`/`zip` rather than a `for i in 0..len` index loop.

## Why
Iterators are the idiomatic traversal and they let the compiler eliminate per-access bounds checks and auto-vectorize. Manual indexing forces a bounds check on every `data[i]`, often blocks those optimizations, and adds off-by-one risk. `zip` also drops the manual length-matching two-slice loops need.

## Do
```rust
fn sum_squares(data: &[i32]) -> i64 {
    data.iter().map(|&x| (x as i64) * (x as i64)).sum()
}

fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

fn double_values(data: &mut [i32]) {
    for x in data.iter_mut() {
        *x *= 2;
    }
}
```

## Don't
```rust
fn sum_squares(data: &[i32]) -> i64 {
    let mut sum = 0i64;
    for i in 0..data.len() {
        sum += (data[i] as i64) * (data[i] as i64);
    }
    sum
}
```

## Notes
Indices are legitimate when you actually need them — use `enumerate()` for a running index, and keep raw indexing for non-sequential access like `data.swap(i * 2, mid + i)`.
