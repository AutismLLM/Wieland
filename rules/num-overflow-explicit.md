---
id: num-overflow-explicit
title: Handle integer overflow explicitly with checked/saturating/wrapping/overflowing
category: numerics
severity: warn
tier: 2
enforceability: gate
tags: [overflow, arithmetic, correctness]
applies_to:
  - "bare +, -, * on integers that can overflow"
  - "arithmetic in release-critical code paths"
rust_edition: "2024"
msrv: null
detect:
  clippy: [arithmetic_side_effects]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#num-overflow-explicit
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Choose an explicit overflow strategy — `checked_*`, `saturating_*`, `wrapping_*`, or `overflowing_*` — instead of relying on bare arithmetic operators.

## Why
Integer overflow panics in debug builds and silently wraps (two's complement) in release builds. Relying on either default is a latent bug: the release build can produce a wrong result with no diagnostic. Picking an explicit variant makes the intended behaviour unmistakable to both the compiler and the next reader.

## Do
```rust
fn add_score(current: u32, delta: u32) -> Option<u32> {
    current.checked_add(delta)
}

fn increment_saturating(c: u8) -> u8 {
    c.saturating_add(1)
}

fn wrapping_sequence(n: u8) -> u8 {
    n.wrapping_add(1)
}

fn add_with_carry(a: u32, b: u32) -> (u32, bool) {
    a.overflowing_add(b)
}
```

## Don't
```rust
fn add_score(current: u32, delta: u32) -> u32 {
    current + delta
}

fn increment_counter(c: u8) -> u8 {
    c + 1
}
```

## Notes
Use `checked_*` when overflow is an error the caller must handle, `saturating_*` when clamping at the type bounds is correct, `wrapping_*` when modular arithmetic is intentional (checksums, ring buffers), and `overflowing_*` when you need the result and the overflow flag together. Every variant exists for add/sub/mul/div/shl/shr on all primitive integer types.
