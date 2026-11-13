---
id: macro-proc-two-crate
title: Put proc-macros in a dedicated proc-macro crate and re-export from the facade
category: macros
severity: warn
tier: 1
enforceability: judge
tags: [proc-macro, crate-layout, workspace, facade]
applies_to:
  - "a library that needs both a derive/attribute macro and ordinary APIs"
  - "a [lib] proc-macro = true crate that also exports types or traits"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-proc-two-crate
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Place procedural macros in a dedicated `proc-macro = true` crate (`mycrate-derive`) and re-export them from a facade crate (`mycrate`) that also holds the ordinary types, traits, and helpers.

## Why
A crate marked `proc-macro = true` compiles for the host and can export only procedural macros — no regular types, traits, or functions. Splitting into an impl crate plus a facade lets users depend on `mycrate` alone, lets generated code reference runtime items through `::mycrate::__private::...` so the impl crate stays invisible, and lets workspace dependency inheritance lock both crates to one version.

## Do
```rust
pub use mycrate_derive::Greet;

pub trait Greet {
    fn greet(&self) -> String;
}

#[doc(hidden)]
pub mod __private {
    pub fn format_greeting(name: &str) -> String {
        format!("hello, {name}")
    }
}
```

## Don't
```rust
#[proc_macro_derive(Greet)]
pub fn derive_greet(input: TokenStream) -> TokenStream {
    /* ... */
}

pub trait Greet {
    fn greet(&self) -> String;
}

pub struct Config;
```

## Notes
The impl crate's `Cargo.toml` needs `[lib] proc-macro = true` and depends on `syn`/`quote`/`proc-macro2`; the facade re-exports the macro so users write `use mycrate::Greet;` or `#[derive(mycrate::Greet)]`. Generated `impl` blocks should reference facade items by absolute path (`::mycrate::__private::...`).
