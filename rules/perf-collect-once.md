---
id: perf-collect-once
title: Don't collect intermediate iterators; collect once at the end
category: performance
severity: warn
tier: 2
enforceability: gate
tags: [performance, allocation, iterators]
applies_to:
  - "collect() followed immediately by another iterator adapter"
  - "collect().len() or collect() only to iterate again"
rust_edition: "2024"
msrv: null
detect:
  clippy: [needless_collect]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-collect-once
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep an iterator chain lazy through every adapter and call `collect` only once, at the point you actually need a concrete collection.

## Why
Each `collect` allocates a new collection and forces a full pass. Collecting into a `Vec` only to immediately re-iterate, count, or filter it again wastes both memory and CPU. A single fused chain does the work in one pass with one (or zero) allocations.

## Do
```rust
fn process_users(users: Vec<User>) -> Vec<String> {
    users.into_iter()
        .filter(|u| u.is_active)
        .filter(|u| u.is_verified)
        .map(|u| u.name)
        .collect()
}

fn count_valid(items: &[Item]) -> usize {
    items.iter().filter(|i| i.is_valid()).count()
}
```

## Don't
```rust
fn process_users(users: Vec<User>) -> Vec<String> {
    let active: Vec<_> = users.into_iter().filter(|u| u.is_active).collect();
    let verified: Vec<_> = active.into_iter().filter(|u| u.is_verified).collect();
    verified.into_iter().map(|u| u.name).collect()
}
```

## Notes
An intermediate `collect` is justified when you must iterate the result multiple times, index it, or sort it — those genuinely need a concrete collection.
