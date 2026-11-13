---
id: macro-proc-no-implied-items
title: Don't have a macro define extra named items the caller never wrote
category: macros
severity: warn
tier: 1
enforceability: judge
tags: [macros, proc-macro, api-design, hygiene]
applies_to:
  - "a #[proc_macro_attribute] or #[proc_macro_derive] that emits a second, unrelated item"
  - "an attribute macro expanding to more top-level items than it was given"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-PROC-IMPLIED-ITEMS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A macro should not conjure extra named items — especially public ones — that do not correspond to anything the caller wrote. If a macro genuinely must introduce a companion item, reuse the caller's own identifier through Rust's namespace system rather than inventing a new name.

## Why
Items a macro invents but the caller never wrote are invisible in the caller's own source: they can collide with names the caller already defined, clash silently across repeated expansions in one module, and are easy to forget to re-export. Reusing the caller's identifier or keeping companions private to the expansion keeps the generated surface predictable and greppable.

## Do
```rust
// The macro reuses the function's own name `foo` for a companion type in
// the type namespace. Because `fn foo` and `struct foo` occupy different
// namespaces, they coexist, and the pair is re-exported together wherever
// `foo` is re-exported -- nothing invisible was introduced.
#[route_handler]
fn foo() {}

// expands to:
fn foo() {}
#[allow(non_camel_case_types)]
struct foo;
impl RouteHandler for foo {}
```

## Don't
```rust
// Bad: the macro invents a brand-new public name `ExtraType` that has no
// counterpart in the caller's source. It can collide with a type the
// caller already defined, clashes silently across multiple expansions in
// the same module, and is easy to forget to re-export.
#[my_macro]
struct UserType;

// expands to:
struct UserType;
pub struct ExtraType;
impl UserType {
    fn foo() -> ExtraType {
        ExtraType
    }
}
```

## Notes
Prefer emitting only the item(s) the caller already named, plus trait impls on them. When a companion item is unavoidable, keep it private to the expansion (a nested module) or piggyback on the caller's identifier via the namespace trick above -- never mint a fresh public name the caller cannot see in their own source.
