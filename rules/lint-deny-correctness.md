---
id: lint-deny-correctness
title: Deny clippy::correctness so definite logic bugs fail the build
category: lints
severity: deny
tier: 3
enforceability: gate
tags: [clippy, correctness, bugs, ci]
applies_to:
  - "lib.rs / main.rs crate attributes"
  - "[lints.clippy] configuration"
  - "code with impossible conditions, NaN comparisons, or use-after-scope"
rust_edition: "2024"
msrv: null
detect:
  clippy: [approx_constant, never_loop, unit_cmp, eq_op, invalid_regex, absurd_extreme_comparisons]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-deny-correctness
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Set `clippy::correctness` to `deny` so code that is outright wrong — impossible conditions, NaN comparisons, imprecise constants, loops that never loop — hard-fails the build instead of warning.

## Why
The correctness group catches code that does not do what its author thinks: `x == f64::NAN` is always false, `x >= 0 && x < 0` can never be true, and `3.14` silently loses precision against `std::f64::consts::PI`. These are not style opinions, they are bugs, so they should be errors and stop the build rather than scroll past as warnings.

## Do
```rust
#![deny(clippy::correctness)]

use std::f64::consts::PI;

fn area(r: f64) -> f64 {
    if r.is_nan() {
        return 0.0;
    }
    PI * r * r
}
```

## Don't
```rust
fn area(r: f64) -> f64 {
    if r == f64::NAN {
        return 0.0;
    }
    3.14 * r * r
}
```

## Notes
`correctness` is denied by default in clippy already; making it explicit (or `correctness = { level = "deny", priority = -1 }` in `Cargo.toml`) documents the intent and prevents a stray `#![allow]` from downgrading it.
