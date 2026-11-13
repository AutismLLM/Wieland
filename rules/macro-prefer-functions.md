---
id: macro-prefer-functions
title: Reach for a macro only when a function or generic cannot express it
category: macros
severity: info
tier: 1
enforceability: judge
tags: [macros, generics, simplicity]
applies_to:
  - "macro_rules! that takes a fixed number of typed arguments"
  - "choosing between a macro and a generic function"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-prefer-functions
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Prefer a generic function or trait over a macro; reach for a macro only when you genuinely need variadics, a DSL, blanket impls over an open set of types, or compile-time boilerplate a function cannot express.

## Why
Macros operate on token streams before type checking, so they bypass type inference, resist IDE navigation, and produce opaque error messages. They also slow incremental compilation and cannot be passed as values. A generic function is clearer, optimizes just as well, is debuggable, and is far easier for contributors to reason about.

## Do
```rust
#[inline]
fn double<T>(x: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    x + x
}

fn main() {
    let n = double(21_i32);
    println!("{n}");
}
```

## Don't
```rust
macro_rules! double {
    ($x:expr) => {
        $x * 2
    };
}

fn main() {
    let n = double!(21);
    println!("{n}");
}
```

## Notes
Legitimate macro cases: truly variadic argument lists (`vec![]`, `println!`), implementing a trait across many unrelated types, embedded DSLs (SQL, HTML, regex), and compile-time format-string validation. Boilerplate a derive could generate belongs in a `#[derive(...)]` proc-macro, not a hand-rolled `macro_rules!`.
