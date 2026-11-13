---
id: macro-rules-hygiene
title: Use $crate for paths in macro_rules! and rely on hygiene for local bindings
category: macros
severity: warn
tier: 2
enforceability: check
tags: [macros, hygiene, paths, exports]
applies_to:
  - "a path to the defining crate's items inside a #[macro_export] macro"
  - "crate:: used in a macro body instead of $crate::"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['=>\s*\{[^}]*\bcrate::', '\bcrate::\w+\s*\(']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-rules-hygiene
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Reference the defining crate's items through `$crate::` inside `macro_rules!` bodies, and rely on macro hygiene to keep macro-introduced `let` bindings from clashing with the caller's.

## Why
`macro_rules!` is hygienic for local bindings, so a `let tmp` inside the macro cannot shadow or be shadowed by the caller's `tmp`. Item paths are not resolved that way: a bare `crate::helper()` resolves relative to the caller's crate, so it silently breaks the moment the macro is used from another crate. `$crate` always expands to the crate that defined the macro, so the path stays correct even when the macro is re-exported.

## Do
```rust
pub fn log_value(v: &str) {
    println!("[log] {v}");
}

#[macro_export]
macro_rules! log {
    ($val:expr) => {
        $crate::log_value(&format!("{:?}", $val));
    };
}
```

## Don't
```rust
pub fn log_value(v: &str) {
    println!("[log] {v}");
}

#[macro_export]
macro_rules! log {
    ($val:expr) => {
        crate::log_value(&format!("{:?}", $val));
    };
}
```

## Notes
Hygiene does not cover identifier tokens the caller passes in as `$name:ident` — those intentionally come from the caller's scope. For items meant only for generated code, combine `$crate::` with a `#[doc(hidden)] pub mod __private` to avoid enlarging the public API.
