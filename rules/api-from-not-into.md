---
id: api-from-not-into
title: Implement From, not Into — From grants the matching Into for free
category: api-design
severity: warn
tier: 2
enforceability: gate
tags: [conversion, from, into]
applies_to:
  - "impl Into<_> for _"
  - "defining a conversion between types"
rust_edition: "2024"
msrv: null
detect:
  clippy: [from_over_into]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-from-not-into
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Define conversions by implementing `From<T>` for the target type; never implement `Into` directly.

## Why
The standard library ships a blanket `impl<T, U: From<T>> Into<U> for T`, so implementing `From` automatically yields the matching `Into`. Implementing `Into` directly gives you only one direction: `T::into()` works but `U::from()` does not, and it reads as non-idiomatic. Clippy's `from_over_into` flags the inverted form. Use `TryFrom` when the conversion can fail.

## Do
```rust
struct UserId(u64);

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

let a = UserId::from(42);
let b: UserId = 42u64.into();
```

## Don't
```rust
struct UserId(u64);

impl Into<UserId> for u64 {
    fn into(self) -> UserId {
        UserId(self)
    }
}
```
