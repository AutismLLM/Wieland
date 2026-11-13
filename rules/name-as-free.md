---
id: name-as-free
title: Use the as_ prefix for free, borrowing reference conversions
category: naming
severity: warn
tier: 2
enforceability: gate
tags: [naming, conversion, api]
applies_to:
  - "methods named as_* that convert one reference to another"
rust_edition: "2024"
msrv: null
detect:
  clippy: [wrong_self_convention]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-as-free
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Reserve the `as_` prefix for cheap, O(1) conversions that borrow `&self` and return a reference into it, never for methods that allocate or consume `self`.

## Why
The `as_`/`to_`/`into_` prefixes encode API cost so callers can reason about it without reading the body: `as_str` is a free reinterpretation, `to_string` allocates, `into_bytes` moves. Naming an allocating or consuming method `as_` breaks that expectation. Clippy's `wrong_self_convention` flags an `as_` method that takes `self` by value instead of `&self`.

## Do
```rust
struct MyString {
    inner: String,
}

impl MyString {
    fn as_str(&self) -> &str {
        &self.inner
    }

    fn to_uppercase(&self) -> String {
        self.inner.to_uppercase()
    }
}
```

## Don't
```rust
struct MyString {
    inner: String,
}

impl MyString {
    fn as_owned(self) -> String {
        self.inner
    }
}
```

## Notes
When the conversion allocates, name it `to_*`; when it consumes `self`, name it `into_*`.
