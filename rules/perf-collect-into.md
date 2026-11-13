---
id: perf-collect-into
title: Reuse a cleared buffer with extend instead of collecting a new one
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, iterators]
applies_to:
  - "collect() into a fresh Vec on every loop iteration"
  - "per-iteration temporary collections in a loop"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-collect-into
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a loop builds a temporary collection each iteration, hoist one buffer, `clear()` it, and `extend()` into it to reuse the allocation.

## Why
`collect()` allocates a fresh collection every time it runs, so a `collect()` inside a loop pays an allocate/free cycle per iteration. Clearing a hoisted buffer keeps its capacity and refills it in place, turning N allocations into one. On stable Rust `extend()` is the idiomatic form (`collect_into` is still nightly-only).

## Do
```rust
fn filter_loop(data: &[Vec<i32>]) {
    let mut buffer = Vec::new();
    for batch in data {
        buffer.clear();
        buffer.extend(batch.iter().filter(|&&x| x > 0).copied());
        process(&buffer);
    }
}
```

## Don't
```rust
fn filter_loop(data: &[Vec<i32>]) {
    for batch in data {
        let filtered: Vec<_> = batch.iter().filter(|&&x| x > 0).copied().collect();
        process(&filtered);
    }
}
```

## Notes
This applies to any `Extend` target — `Vec`, `HashSet`, `VecDeque`, `String`. If the temporary escapes the loop or must be owned by the caller, a fresh `collect()` is the right call.
