---
id: macro-export-crate-path
title: Export declarative macros with #[macro_export] and a path import, not #[macro_use]
category: macros
severity: warn
tier: 2
enforceability: check
tags: [macros, exports, edition-2018, imports]
applies_to:
  - "#[macro_export]"
  - "#[macro_use] extern crate"
  - "a declarative macro meant to be used by other crates"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['#\[macro_use\]', 'extern crate']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-export-crate-path
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Export a declarative macro with `#[macro_export]` and let callers import it by path (`use mycrate::my_macro;`) rather than the legacy `#[macro_use] extern crate`.

## Why
`#[macro_export]` lifts a macro to the crate root so it imports like any other item, and path imports are explicit, order-independent, and understood by `rustfmt` and IDEs. The legacy `#[macro_use] extern crate mylib;` dumps every macro into the caller's global namespace and is sensitive to declaration order. Path imports are the Rust 2018+ default and `#[macro_use]` is only needed to support pre-2018 consumers.

## Do
```rust
#[macro_export]
macro_rules! greet {
    ($name:expr) => {
        $crate::print_greeting($name);
    };
}

pub fn print_greeting(name: &str) {
    println!("hello, {name}");
}
```

## Don't
```rust
macro_rules! greet {
    ($name:expr) => {
        println!("hello, {}", $name);
    };
}

#[macro_use]
extern crate mylib;
```

## Notes
`#[macro_export]` always places the macro at the crate root regardless of the module the `macro_rules!` block lives in; use a `pub use crate::greet;` re-export if you want it reachable under a module path too.
