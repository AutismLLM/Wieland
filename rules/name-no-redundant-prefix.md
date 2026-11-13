---
id: name-no-redundant-prefix
title: Keep identifiers short; don't bake module or crate names into item prefixes
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, brevity, modules]
applies_to:
  - "type or module names that repeat their parent module's name"
  - "compound identifiers longer than two short words"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SHORT-NAMES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep item identifiers to at most two short words, prefer abbreviations over spelled-out compounds, and let the enclosing module supply context instead of repeating it in the name.

## Why
Repeating the module or crate name in every item (`foo::FooId`) lengthens call sites without adding information the path already carries. The module system supplies that context, so short names read better at the use site while the compiler still disambiguates `foo::Id` from `bar::Id`.

## Do
```rust
mod foo {
    pub struct Id(u64);
}

mod bar {
    pub struct Id(u64);
}

fn convert(id: foo::Id) -> bar::Id {
    bar::Id(id.0)
}

struct AppConfig {
    port: u16,
}

type CallbackFn = fn(i32) -> i32;
```

## Don't
```rust
mod foo {
    pub struct FooId(u64);
}

mod bar {
    pub struct BarId(u64);
}

fn convert(id: foo::FooId) -> bar::BarId {
    bar::BarId(id.0)
}

struct GlobalApplicationConfig {
    port: u16,
}

type CallbackFunction = fn(i32) -> i32;
```

## Notes
Break this rule where it makes local sense — a re-exported type that leaves its module behind may need to reclaim context in its name — but treat such exceptions as exceptional per crate, not the default.
