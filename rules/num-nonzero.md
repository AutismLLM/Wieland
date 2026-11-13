---
id: num-nonzero
title: Use `NonZero*` types to forbid zero and unlock the niche optimization
category: numerics
severity: info
tier: 1
enforceability: judge
tags: [types, invariants, layout]
applies_to:
  - "integer field or parameter where zero is invalid"
  - "runtime assert that a value is non-zero"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#num-nonzero
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When zero is an invalid value, use a `NonZero*` type so the invariant is enforced at construction and `Option<NonZero*>` costs no extra space.

## Why
`NonZeroU32` and its siblings make zero unrepresentable at the type level: you can only build one through `NonZeroU32::new(n)`, which returns `Option<NonZeroU32>`. This pushes the zero-check to the construction site and removes defensive checks everywhere else. The compiler also uses the zero bit-pattern as a niche, so `Option<NonZeroU32>` is the same size as `u32` — the `Option` tag is free.

## Do
```rust
use std::num::NonZeroU32;

fn divide(numerator: u32, denominator: NonZeroU32) -> u32 {
    numerator / denominator.get()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WidgetId(NonZeroU32);

impl WidgetId {
    pub fn new(id: u32) -> Option<Self> {
        NonZeroU32::new(id).map(WidgetId)
    }

    pub fn get(self) -> u32 {
        self.0.get()
    }
}
```

## Don't
```rust
fn divide(numerator: u32, denominator: u32) -> u32 {
    assert!(denominator != 0, "denominator must not be zero");
    numerator / denominator
}

struct Widget {
    id: u32,
}
```

## Notes
`NonZero*` does not implement `Add`/`Sub` directly (the result could be zero): extract with `.get()`, compute, and reconstruct with `NonZeroU32::new(result)?`. Use `NonZeroU32::new(n).expect(...)` only at well-verified program boundaries, never in production fallible paths.
