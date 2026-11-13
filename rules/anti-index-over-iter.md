---
id: anti-index-over-iter
title: Iterate with iterators instead of manual index ranges
category: anti-patterns
severity: warn
tier: 2
enforceability: gate
tags: [iterators, idiom, performance]
applies_to:
  - "for i in 0..slice.len() { slice[i] }"
rust_edition: "2024"
msrv: null
detect:
  clippy: [needless_range_loop]
  grep: ['for .* in 0\.\.']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-index-over-iter
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Walk a slice with `.iter()`, `.iter_mut()`, or `.zip()` instead of a `for i in 0..len` loop that indexes on every access.

## Why
Manual index loops force a bounds check on each `slice[i]`, block the auto-vectorizer, and invite off-by-one errors. Iterators elide the bounds checks, are friendlier to SIMD, and express intent directly. `zip` also handles length mismatches for you.

## Do
```rust
fn sum_squares(data: &[i32]) -> i64 {
    data.iter().map(|&x| (x as i64) * (x as i64)).sum()
}
```

## Don't
```rust
fn sum_squares(data: &[i32]) -> i64 {
    let mut result = 0i64;
    for i in 0..data.len() {
        result += (data[i] as i64) * (data[i] as i64);
    }
    result
}
```

## Notes
Keep indices when you genuinely need them: `enumerate()` for a running index, `step_by` for strided access, or nested `i`/`j` loops over a matrix.
