---
id: mem-reuse-collections
title: Clear and reuse a collection across loop iterations instead of reallocating
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, hot-path]
applies_to:
  - "Vec::new or String::new inside a loop body"
  - "temporary buffer rebuilt each iteration"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-reuse-collections
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a loop builds a temporary collection each iteration, hoist it out of the loop and `clear()` it at the top instead of allocating a new one every pass.

## Why
Constructing a fresh `Vec`, `String`, or `HashMap` inside a hot loop allocates and deallocates on every iteration, generating steady allocator pressure. `clear()` resets the length to zero while retaining the existing capacity, so subsequent iterations reuse the same allocation. This turns N allocations into one.

## Do
```rust
fn process_batches(batches: &[Batch]) -> Vec<Summary> {
    let mut results = Vec::with_capacity(batches.len());
    let mut temp = Vec::new();

    for batch in batches {
        temp.clear();
        for item in &batch.items {
            temp.push(transform(item));
        }
        results.push(aggregate(&temp));
    }

    results
}
```

## Don't
```rust
fn process_batches(batches: &[Batch]) -> Vec<Summary> {
    let mut results = Vec::new();

    for batch in batches {
        let mut temp = Vec::new();
        for item in &batch.items {
            temp.push(transform(item));
        }
        results.push(aggregate(&temp));
    }

    results
}
```

## Notes
Allocate fresh when ownership of each collection must be transferred out (e.g. pushed into a result vector) or when a value is moved into another thread, since a reused buffer cannot be shared.
