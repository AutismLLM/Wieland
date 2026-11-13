---
id: macro-private-helpers
title: Hide macro-generated helpers behind a #[doc(hidden)] pub mod __private
category: macros
severity: warn
tier: 1
enforceability: judge
tags: [macros, api-surface, semver, proc-macro]
applies_to:
  - "an exported macro that calls helper functions or types at the call site"
  - "a derive crate whose generated code references facade-crate helpers"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-private-helpers
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Route the helper functions, types, and traits that your exported macros call through a `#[doc(hidden)] pub mod __private`, never through the bare public crate root.

## Why
Exported macros expand at the caller's site, so any item they reference must be public. Placing those helpers directly in the crate's public API pollutes the surface with implementation details and freezes them under semver, making any rename or removal a breaking change. A `#[doc(hidden)] pub mod __private` keeps them reachable for expansion while hiding them from docs and signalling that they are unstable internals. This is the pattern `serde` and `thiserror` use.

## Do
```rust
#[doc(hidden)]
pub mod __private {
    pub use crate::helpers::format_value;
}

mod helpers {
    pub fn format_value(v: &dyn std::fmt::Debug) -> String {
        format!("{v:?}")
    }
}

#[macro_export]
macro_rules! debug_print {
    ($val:expr) => {
        println!("{}", $crate::__private::format_value(&$val));
    };
}
```

## Don't
```rust
pub fn __format_value(v: &dyn std::fmt::Debug) -> String {
    format!("{v:?}")
}

#[macro_export]
macro_rules! debug_print {
    ($val:expr) => {
        println!("{}", $crate::__format_value(&$val));
    };
}
```

## Notes
Use the conventional double-underscore name `__private`, always pair it with `#[doc(hidden)]`, and reference items as `$crate::__private::...` inside macros. Document in your README or CHANGELOG that `__private` is exempt from semver guarantees.
