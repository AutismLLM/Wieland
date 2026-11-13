---
id: mem-assert-type-size
title: Guard hot types against accidental size growth with static size assertions
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, layout, invariant]
applies_to:
  - "types stored in large collections"
  - "FFI or wire-format structs with a fixed size"
  - "std::mem::size_of"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-assert-type-size
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For types instantiated in bulk or bound to a binary layout, pin their size with a compile-time `const _: () = assert!(size_of::<T>() == N);` so accidental growth breaks the build.

## Why
Adding one field to a struct stored in a million-element collection can silently add megabytes of memory, with no warning and nothing to trigger a review. A static size assertion turns that silent change into a compile error, forcing the size change to be intentional. It is essential for FFI and wire-format types whose size is part of a contract.

## Do
```rust
struct Event {
    timestamp: u64,
    kind: EventKind,
    payload: [u8; 32],
}

const _: () = assert!(std::mem::size_of::<Event>() == 48);
```

## Don't
```rust
struct Event {
    timestamp: u64,
    kind: EventKind,
    payload: [u8; 32],
    metadata: String,
}
```

## Notes
Skip it for rarely-instantiated types like a top-level config held once — its size does not matter, and the assertion is just noise.
