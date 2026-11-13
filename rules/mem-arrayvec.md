---
id: mem-arrayvec
title: Use ArrayVec for fixed-capacity collections that never heap-allocate
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, no-std]
applies_to:
  - "Vec with a known hard upper bound"
  - "embedded or real-time code that forbids heap allocation"
  - "arrayvec"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-arrayvec
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a collection has a hard compile-time capacity limit and must never touch the heap, use `ArrayVec<T, N>` so all elements live inline.

## Why
`ArrayVec` stores its elements in a fixed inline buffer and guarantees no heap allocation, unlike `Vec` (always heap) or `SmallVec` (spills to heap past `N`). This matters for embedded, `no_std`, and real-time code where allocation is forbidden or must be deterministic. Exceeding capacity returns an error from `try_push` or panics, making the bound explicit.

## Do
```rust
use arrayvec::ArrayVec;

fn parse_options(input: &str) -> ArrayVec<u32, 8> {
    let mut options = ArrayVec::new();
    for part in input.split(',') {
        if options.try_push(parse_option(part)).is_err() {
            break;
        }
    }
    options
}
```

## Don't
```rust
fn parse_options(input: &str) -> Vec<u32> {
    let mut options = Vec::new();
    for part in input.split(',').take(8) {
        options.push(parse_option(part));
    }
    options
}
```

## Notes
Avoid `ArrayVec` when the size varies widely (you cannot pick a sane `N`) or when the capacity is very large, since the whole buffer lives on the stack. Reach for `SmallVec` or `Vec` in those cases.
