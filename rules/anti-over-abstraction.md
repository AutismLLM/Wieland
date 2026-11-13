---
id: anti-over-abstraction
title: Start with concrete types; add generics only when they earn their keep
category: anti-patterns
severity: info
tier: 1
enforceability: judge
tags: [design, generics, simplicity]
applies_to:
  - "a generic function with a single call site"
  - "a trait with only one implementor"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-over-abstraction
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write concrete code first; introduce generics and traits only when a real second use case demands them.

## Why
Generics and traits cost compile time, binary size, and cognitive load. Making everything generic "for flexibility" adds all of that with no payoff when only one type will ever be used. Type-parameter soup and marker-trait towers make code harder to read and change, not easier.

## Do
```rust
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
```

## Don't
```rust
fn add<T, U, R>(a: T, b: U) -> R
where
    T: Into<R>,
    U: Into<R>,
    R: std::ops::Add<Output = R>,
{
    a.into() + b.into()
}
```

## Notes
Generalize when two or more concrete types share behavior, when you are writing a public library API, or when static dispatch is required for performance. Following the rule of three — wait for a third similar implementation before abstracting — keeps speculative flexibility out of the codebase.
