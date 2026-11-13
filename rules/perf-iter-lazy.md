---
id: perf-iter-lazy
title: Keep iterators lazy and collect only when you need a collection
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, iterators]
applies_to:
  - "collect() in the middle of a transformation pipeline"
  - "collecting a filtered result only to test is_empty or first"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-iter-lazy
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Chain adapters lazily and let consuming methods like `any`, `find`, `count`, and `sum` do the work, instead of collecting mid-pipeline.

## Why
Rust iterators compute on demand, which enables single-pass processing, avoids intermediate allocations, and lets consumers short-circuit. Calling `collect` early forces full evaluation and allocates a collection you may only glance at. Using the right consuming adapter often removes the allocation entirely and stops as soon as the answer is known.

## Do
```rust
fn process(data: Vec<i32>) -> Vec<i32> {
    data.into_iter()
        .filter(|x| *x > 0)
        .map(|x| x * 2)
        .take(10)
        .collect()
}

fn has_positive(data: &[i32]) -> bool {
    data.iter().any(|&x| x > 0)
}
```

## Don't
```rust
fn has_positive(data: &[i32]) -> bool {
    let positives: Vec<_> = data.iter().filter(|&&x| x > 0).collect();
    !positives.is_empty()
}
```

## Notes
`filter`, `map`, `take`, `zip`, `flat_map`, and `enumerate` stay lazy; `collect`, `count`, `sum`, `any`, `find`, and `for_each` consume. Reach for a consuming method that matches your actual question rather than materializing the whole sequence.
