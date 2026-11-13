---
id: own-clone-explicit
title: Keep Clone explicit for types where duplication has real cost
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [clone, allocation, cost-visibility]
applies_to:
  - "type carrying String, Vec, Box, or other heap data"
  - "repeated .clone() into an existing buffer"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-clone-explicit
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give heap-carrying types `Clone` (not `Copy`) so every duplication is a visible `.clone()` call, and use `clone_from` when overwriting a value you already hold to reuse its allocation.

## Why
`Copy` is implicit and signals "free"; `Clone` forces an explicit call that flags a possible allocation or deep copy at the call site. That visibility is exactly what you want for `String`, `Vec`, and `Box`. `clone_from` goes further: it reuses the destination's existing capacity instead of dropping it and allocating anew.

## Do
```rust
fn refresh(buffer: &mut String, source: &String) {
    buffer.clone_from(source);
}
```

## Don't
```rust
fn refresh(buffer: &mut String, source: &String) {
    *buffer = source.clone();
}
```

## Notes
Borrowing beats cloning when you do not need ownership — take `&str`/`&[T]`. For cheap shared handles (`Arc`, `Rc`) `.clone()` is just a refcount bump and is fine in a loop.
