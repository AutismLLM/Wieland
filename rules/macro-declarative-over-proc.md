---
id: macro-declarative-over-proc
title: Prefer a macro_rules! macro over a proc macro when either can do the job
category: macros
severity: info
tier: 1
enforceability: judge
tags: [macros, proc-macro, compile-time, tooling]
applies_to:
  - "choosing between a #[proc_macro_attribute] and a macro_rules! macro"
  - "an attribute or derive macro that only needs to wrap or repeat tokens"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-EXAMPLE-OVER-PROC
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a `macro_rules!` macro can do the job, prefer it over a procedural macro.

## Why
A `macro_rules!` expansion can be inspected with `cargo expand` or an IDE hover with no extra tooling, and it compiles far faster because it needs no `syn`/`quote`/`proc-macro2` dependency chain or separate compilation unit. Proc macros are strictly more powerful — they can parse arbitrary syntax and inspect types — but that power is wasted, and its cost paid, on jobs that are really just "generate this fixed shape of code from these tokens."

## Do
```rust
macro_rules! make_new_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct $name(u64);

        impl $name {
            fn new(v: u64) -> Self {
                Self(v)
            }
        }
    };
}

make_new_id!(UserId);
make_new_id!(OrderId);
```

## Don't
```rust
// Attribute proc macro pulls in syn/quote/proc-macro2 and a separate
// proc-macro crate, and its expansion can't be inspected without
// cargo-expand, even though it only needs to stamp out a fixed shape.
#[make_new_id]
struct UserId;

#[make_new_id]
struct OrderId;
```

## Notes
Reach for a proc macro once you genuinely need to inspect or transform arbitrary Rust syntax (generics, where-clauses, field attributes) or drive a derive across an open set of user-defined types — jobs `macro_rules!` token matching cannot express.
