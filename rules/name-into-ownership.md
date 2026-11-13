---
id: name-into-ownership
title: Use the into_ prefix for ownership-consuming conversions
category: naming
severity: warn
tier: 2
enforceability: gate
tags: [naming, conversion, ownership]
applies_to:
  - "methods that consume self and return an owned value"
rust_edition: "2024"
msrv: null
detect:
  clippy: [wrong_self_convention]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-into-ownership
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Prefix a method that takes `self` by value and returns an owned value with `into_`.

## Why
`into_` tells the caller the receiver is moved and no longer usable, so "use after move" errors become predictable rather than surprising. Naming such a method `as_` or `get_` hides the ownership transfer; clippy's `wrong_self_convention` flags an `as_` method that consumes `self`.

## Do
```rust
struct Inner;

struct Wrapper {
    inner: Inner,
}

impl Wrapper {
    fn into_inner(self) -> Inner {
        self.inner
    }
}
```

## Don't
```rust
struct Inner;

struct Wrapper {
    inner: Inner,
}

impl Wrapper {
    fn as_inner(self) -> Inner {
        self.inner
    }
}
```

## Notes
The transfer is usually cheap (no allocation); the point of the prefix is the move, not the cost.
