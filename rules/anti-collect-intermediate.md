---
id: anti-collect-intermediate
title: Keep iterator chains lazy and collect only once at the end
category: anti-patterns
severity: warn
tier: 2
enforceability: gate
tags: [iterators, allocation, performance]
applies_to:
  - "a .collect() immediately re-iterated by another adaptor"
  - "collecting a Vec just to check length or emptiness"
rust_edition: "2024"
msrv: null
detect:
  clippy: [needless_collect]
  grep: ['\.collect::<Vec']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-collect-intermediate
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Chain iterator adaptors lazily and call `.collect()` only at the final step, not between stages.

## Why
Each `.collect()` allocates a new collection and forces a full pass. Collecting intermediate results breaks iterator fusion, so a three-stage pipeline pays three allocations and three passes instead of one. Terminal operations like `any`, `count`, `sum`, and `find` need no collection at all.

## Do
```rust
fn process(data: Vec<i32>) -> Vec<i32> {
    data.into_iter()
        .filter(|x| *x > 0)
        .map(|x| x * 2)
        .filter(|x| *x < 100)
        .collect()
}
```

## Don't
```rust
fn process(data: Vec<i32>) -> Vec<i32> {
    let step1: Vec<_> = data.into_iter().filter(|x| *x > 0).collect();
    let step2: Vec<_> = step1.into_iter().map(|x| x * 2).collect();
    step2.into_iter().filter(|x| *x < 100).collect()
}
```

## Notes
Collect when you genuinely need a concrete collection: to iterate twice, to sort, or for random access. To defer the choice, return `impl Iterator<Item = _>` and let the caller collect only if needed.
