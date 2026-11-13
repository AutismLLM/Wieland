---
id: num-float-compare
title: Compare floats with a tolerance and order them with total_cmp
category: numerics
severity: warn
tier: 2
enforceability: gate
tags: [correctness, numerics, floating-point]
applies_to:
  - "f32 or f64 compared with =="
  - "partial_cmp().unwrap() in sort_by"
  - "NaN handling"
rust_edition: "2024"
msrv: null
detect:
  clippy: [float_cmp]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#num-float-compare
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Compare floats for equality with an epsilon tolerance, and sort them with `total_cmp` rather than `partial_cmp().unwrap()`.

## Why
Floating-point arithmetic is not exact: `0.1 + 0.2 == 0.3` is `false` in Rust and every IEEE 754 language, because those values are not representable in binary. `NaN != NaN` by the standard, so `==` involving `NaN` is always `false`. And `f64::partial_cmp` returns `None` on `NaN`, so `sort_by(|a, b| a.partial_cmp(b).unwrap())` panics on the inconsistent order. An epsilon comparison and `total_cmp` (stable since 1.62) sidestep all three traps.

## Do
```rust
fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

fn is_unit_length(x: f64, y: f64) -> bool {
    approx_eq((x * x + y * y).sqrt(), 1.0, 1e-9)
}

fn sort_scores(scores: &mut [f64]) {
    scores.sort_by(|a, b| a.total_cmp(b));
}
```

## Don't
```rust
fn is_unit_length(x: f64, y: f64) -> bool {
    (x * x + y * y).sqrt() == 1.0
}

fn sort_scores(scores: &mut Vec<f64>) {
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
}
```

## Notes
An absolute epsilon is simple but wrong for very large or very small magnitudes; a relative epsilon `(a - b).abs() / a.abs().max(b.abs()) < epsilon` is more robust but must handle the zero case. `total_cmp` defines a strict total order and never panics. Bit-exact `==` is intentionally kept for cases like detecting whether a stored value changed — document such uses.
