---
id: mem-smallvec
title: Use SmallVec for collections that are usually small but can occasionally grow
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, hot-path]
applies_to:
  - "Vec that usually holds only a few elements"
  - "path components, function arguments, AST children, error lists"
  - "smallvec"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-smallvec
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a collection almost always holds a small number of elements but must still be able to grow, use `SmallVec<[T; N]>` to keep the common case on the stack.

## Why
`SmallVec` stores up to `N` elements inline and only spills to the heap when it grows past `N`, eliminating heap allocation for the common small case while still allowing unbounded growth. A plain `Vec` allocates even for one or two elements. The cost is a slightly larger type and a branch on each access to check inline-vs-heap, so profile to confirm the win.

## Do
```rust
use smallvec::SmallVec;

fn get_path_components(path: &str) -> SmallVec<[&str; 8]> {
    path.split('/').collect()
}
```

## Don't
```rust
fn get_path_components(path: &str) -> Vec<&str> {
    path.split('/').collect()
}
```

## Notes
Pick `N` from your measured data distribution, not a guess. If there is a hard maximum and heap fallback is unacceptable, use `ArrayVec`; if you must avoid `unsafe`, use `TinyVec` instead.
