---
id: api-impl-rangebounds
title: Accept impl RangeBounds<T> for functions that select an arbitrary range
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [ranges, ergonomics, generics]
applies_to:
  - "fn taking separate low/high parameters to describe a range"
  - "fn taking a (T, T) tuple to describe a range"
  - "fn that could accept an open-ended or full range"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-IMPL-RANGEBOUNDS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Express a range parameter with a real range type, and when the function can operate on any kind of range, accept `impl RangeBounds<T>` rather than a concrete `Range<T>` — never hand-rolled `(low, high)` parameters or tuples.

## Why
Two loose `usize` parameters or a `(usize, usize)` tuple carry no meaning about which bound is inclusive, and let a caller swap low and high without any compiler feedback. A real `Range<T>` fixes that ambiguity, but pins callers to exactly `a..b`; accepting `impl RangeBounds<T>` instead lets the same function take `a..b`, `a..`, `..b`, and `..`, matching how the standard library's own range-taking APIs (like slice indexing) behave.

## Do
```rust
use std::ops::RangeBounds;

fn select_any(r: impl RangeBounds<usize>) {
    let _ = r;
}

select_any(1..3);
select_any(1..);
select_any(..);
```

## Don't
```rust
// Ambiguous bounds, and callers cannot express an open-ended range.
fn select_range(low: usize, high: usize) {
    let _ = (low, high);
}

fn select_range_tuple(range: (usize, usize)) {
    let _ = range;
}
```

## Notes
If the function genuinely only ever operates on a bounded, half-open range, a concrete `Range<T>` parameter is fine and clearer than `impl RangeBounds<T>`; reach for the trait bound only when the function can meaningfully act on an open or full range too.
