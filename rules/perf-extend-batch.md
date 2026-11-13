---
id: perf-extend-batch
title: Insert many elements with extend, not a push loop
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, collections]
applies_to:
  - "a loop that push()es elements from an iterator or collection"
  - "combining two collections element by element"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-extend-batch
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
To add many elements at once, call `extend()` (or `collect`) rather than pushing them one at a time in a loop.

## Why
`extend()` reads the iterator's `size_hint` and reserves capacity up front, then inserts in a single pass. A hand-rolled `push` loop can trigger several reallocations as the collection grows, each copying the existing contents to a larger buffer. `extend` is both faster and clearer about intent.

## Do
```rust
fn collect_results(sources: Vec<Source>) -> Vec<Item> {
    let mut results = Vec::new();
    for source in sources {
        results.extend(source.get_results());
    }
    results
}

fn combine(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.extend(b);
    a
}
```

## Don't
```rust
fn collect_results(sources: Vec<Source>) -> Vec<Item> {
    let mut results = Vec::new();
    for source in sources {
        for result in source.get_results() {
            results.push(result);
        }
    }
    results
}
```

## Notes
Pair `extend` with `Vec::with_capacity` when the total size is known for a single guaranteed allocation. For `Copy` slices use `extend_from_slice`; to move another `Vec` empty use `append`.
