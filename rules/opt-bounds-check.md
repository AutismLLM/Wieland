---
id: opt-bounds-check
title: Eliminate bounds checks in hot loops with iterators and slice patterns
category: optimization
severity: warn
tier: 2
enforceability: gate
tags: [performance, hot-path, iterators, bounds-check]
applies_to:
  - "for i in 0..slice.len() with slice[i] indexing"
  - "tight numeric loops over slices"
rust_edition: "2024"
msrv: null
detect:
  clippy: [needless_range_loop]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-bounds-check
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In hot paths, index through iterators, `windows`, `chunks_exact`, `zip`, and slice patterns instead of `slice[i]`, so the compiler can drop bounds checks and vectorize.

## Why
Every `slice[i]` compiles to a bounds check plus a branch; in a tight loop those branches thwart vectorization and add measurable overhead. Iterator combinators and slice patterns carry their length invariant with them, so the compiler proves access is in-bounds once and emits straight-line, vectorizable code. Elimination is not guaranteed — verify hot code with generated assembly.

## Do
```rust
fn sum_products(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
```

## Don't
```rust
fn sum_products(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}
```

## Notes
For genuinely random or infrequent access, keep the checked `slice.get(i)` — the check is unavoidable and its cost is negligible off the hot path. Reach for `get_unchecked` only behind proven asserts with a `// SAFETY:` comment.
