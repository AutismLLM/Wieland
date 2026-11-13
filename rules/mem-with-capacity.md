---
id: mem-with-capacity
title: Preallocate collections you will grow to a known size
category: memory
severity: warn
tier: 2
enforceability: check
tags: [performance, allocation, hot-path]
applies_to:
  - "Vec::new"
  - "HashMap::new"
  - "String::new"
  - "loop that pushes a known number of items"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['Vec::new\(\)', 'HashMap::new\(\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-with-capacity
  - microsoft/rust-guidelines#allocations
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When the final size of a collection is known or bounded before you fill it, construct it with `with_capacity` instead of growing it push-by-push.

## Why
A default `Vec`/`HashMap`/`String` starts empty and reallocates as it grows — repeatedly copying its contents to a larger buffer (roughly `log2(n)` reallocations for `n` pushes). In a hot path this is wasted memcpy and allocator traffic that a single up-front allocation removes. It also communicates intent: the size is known.

## Do
```rust
let mut ids = Vec::with_capacity(events.len());
for e in &events {
    ids.push(e.id);
}
```

## Don't
```rust
let mut ids = Vec::new();
for e in &events {
    ids.push(e.id);
}
```

## Notes
Skip it when the size is genuinely unknown or tiny — a fixed 3-element vector gains nothing, and a wrong capacity guess can waste memory. Prefer `iter().collect()` when it applies; `collect` already reserves from the iterator's size hint.
