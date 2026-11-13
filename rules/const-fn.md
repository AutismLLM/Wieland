---
id: const-fn
title: Make functions const fn when they can run at compile time
category: const-eval
severity: warn
tier: 2
enforceability: check
tags: [compile-time, api, zero-cost]
applies_to:
  - "pure function used as an array length"
  - "function used in a const or static initializer"
  - "arithmetic or bitwise helper without heap or dynamic dispatch"
rust_edition: "2024"
msrv: null
detect:
  clippy: [missing_const_for_fn]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#const-fn
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Mark a pure, simple function `const fn` so it can also be evaluated in const contexts at zero cost.

## Why
A `const fn` can be called wherever a constant is required — array lengths, `const`/`static` initializers, const-generic arguments — as well as at runtime, so `const` widens where the function is usable for free. Calls in const contexts are evaluated during compilation and disappear from the binary. Stable Rust already supports most arithmetic, bitwise ops, conditionals, and slice operations in `const fn`; the main restrictions are heap allocation and most trait method calls. Adding `const` is a backwards-compatible change for library authors.

## Do
```rust
const fn header_len() -> usize {
    4
}

const fn align_up(n: usize, align: usize) -> usize {
    (n + align - 1) & !(align - 1)
}

let buf = [0u8; header_len()];
const MASK: u32 = 0xFF00_FF00;
static HEADER: [u8; header_len()] = [0u8; header_len()];
const ALIGNED: usize = align_up(13, 8);
```

## Don't
```rust
fn header_len() -> usize {
    4
}

fn make_buf() -> [u8; 8] {
    [0u8; header_len()]
}
```

## Notes
Start with `const fn` for any pure function without heap allocation or dynamic dispatch; you can drop `const` later if you need a capability that is not yet const-stable. Avoid `const fn` for functions that depend on unstable const features not yet stabilized on your target channel.
