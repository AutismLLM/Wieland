---
id: opt-inline-small
title: Add #[inline] to small hot functions, especially across crate boundaries
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, inlining, attributes, cross-crate]
applies_to:
  - "small public functions called in tight loops"
  - "generic helpers exposed from a library crate"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-inline-small
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Add `#[inline]` to small, frequently-called functions — most importantly public ones a downstream crate calls in a tight loop.

## Why
Call overhead (stack setup, register saves, the jump) can dominate a one-line function, and inlining removes it while exposing the body to further optimization. Within a crate the compiler often inlines automatically, but without LTO a function body is not visible to other crates unless it carries `#[inline]`, so cross-crate hot calls stay un-inlined by default. `#[inline]` is a hint, not a force — it fixes the cross-crate case without the downsides of `always`.

## Do
```rust
#[inline]
fn is_ascii_digit(b: u8) -> bool {
    b.is_ascii_digit()
}
```

## Don't
```rust
fn is_ascii_digit(b: u8) -> bool {
    b.is_ascii_digit()
}
```

## Notes
Do not `#[inline]` large functions — inlining a 100-line body at every call site bloats code and hurts the instruction cache. For same-crate code the compiler usually decides well on its own; verify hot paths with assembly rather than annotating blindly.
