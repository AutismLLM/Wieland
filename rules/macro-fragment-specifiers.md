---
id: macro-fragment-specifiers
title: Capture macro arguments with precise fragment specifiers, not raw :tt
category: macros
severity: warn
tier: 2
enforceability: check
tags: [macros, fragment-specifiers, diagnostics]
applies_to:
  - "macro_rules! arm capturing $x:tt"
  - "a declarative macro that re-parses captured tokens by hand"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['\$\w+:tt']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-fragment-specifiers
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Capture each macro argument with the most specific fragment specifier that fits (`:expr`, `:ty`, `:ident`, `:pat`, ...) instead of slurping tokens as `:tt`.

## Why
A precise specifier tells the compiler and the reader exactly what syntactic category the arm expects, so mismatches produce targeted errors ("expected expression") instead of "no rules expected token", and IDE tooling can reason about the input. Raw `:tt` accepts anything and forces you to re-validate or re-parse by hand, pushing failures to expansion time where they are far harder to diagnose.

## Do
```rust
macro_rules! debug_val {
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e);
    };
}

fn main() {
    debug_val!(1 + 2);
}
```

## Don't
```rust
macro_rules! debug_val {
    ($($t:tt)*) => {
        println!("{} = {:?}", stringify!($($t)*), $($t)*);
    };
}

fn main() {
    debug_val!(1 + 2);
    debug_val!(let x = 1);
}
```

## Notes
`:tt` is the right tool for genuine token-tree combinators and passthrough macros; the point is to reach for it last, not never. Mind the follow-set restriction: after `:expr`, `:ty`, or `:pat` only a limited set of tokens (`=>`, `,`, `;`, `|`) may follow, so plan separators accordingly.
