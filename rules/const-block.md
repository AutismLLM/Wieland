---
id: const-block
title: Use inline const blocks for compile-time evaluation and assertions
category: const-eval
severity: info
tier: 1
enforceability: judge
tags: [compile-time, assertions, correctness]
applies_to:
  - "runtime assert! on a constant invariant"
  - "magic numbers computed at runtime"
  - "compile-time bounds checks on const generics"
rust_edition: "2024"
msrv: "1.79"
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#const-block
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Reach for an inline `const { }` block when you want a one-off compile-time guarantee or an inlined value without a named `const` item.

## Why
A `const { }` block (stabilized in Rust 1.79) forces its expression to be evaluated at compile time even inside a regular function. This turns invariant checks into build failures instead of runtime panics, inlines precomputed values at the exact call site, and lets you initialize arrays whose elements each need a constant expression. Catching a violation at compile time gives a clearer error and zero runtime cost.

## Do
```rust
const SIZE: usize = 64;

fn process(buf: &[u8]) {
    const { assert!(SIZE.is_power_of_two(), "SIZE must be a power of two") };
    assert!(buf.len() <= SIZE);
}

fn magic_header() -> u32 {
    const { 0xDEAD_BEEFu32.swap_bytes() }
}

struct Packet<const HDR: usize, const BODY: usize>;

impl<const HDR: usize, const BODY: usize> Packet<HDR, BODY> {
    fn new() -> Self {
        const { assert!(HDR + BODY <= 1500, "packet exceeds ethernet MTU") };
        Packet
    }
}
```

## Don't
```rust
const SIZE: usize = 64;

fn process(buf: &[u8]) {
    assert!(SIZE.is_power_of_two(), "SIZE must be a power of two");
    assert!(buf.len() <= SIZE);
}

fn header() -> [u8; 4] {
    [0u8; 4]
}
```

## Notes
`const { expr }` is an expression usable anywhere an expression is valid: function bodies, match arms, array initializers, default values. For long-lived or widely shared values, prefer a named `const ITEM: T = expr;`; reach for `const { }` when the guarantee or value is one-off and local.
