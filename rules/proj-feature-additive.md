---
id: proj-feature-additive
title: Design Cargo features to be strictly additive, never subtractive
category: project-layout
severity: warn
tier: 1
enforceability: judge
tags: [cargo, features, api-stability]
applies_to:
  - "[features] in Cargo.toml"
  - "a feature that disables or changes existing behavior"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-feature-additive
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A Cargo feature must only add capability — new trait impls, optional dependencies, extra integrations — and never remove or alter existing behavior.

## Why
Cargo unifies features across the whole dependency graph: if any crate enables a feature, every consumer of that crate gets it. A feature that subtracts behavior (like a `no_std` toggle) will break crates relying on the baseline the moment a third dependency turns it on. Make `std` the additive opt-in with `no_std` as the baseline; mutually exclusive features cannot be enforced by Cargo.

## Do
```toml
[features]
default = ["std"]
std = []
serde = ["dep:serde"]
tokio = ["dep:tokio"]

[dependencies]
serde = { version = "1", optional = true }
tokio = { version = "1", optional = true }
```

## Don't
```toml
[features]
# enabling "no_std" REMOVES behavior — non-additive, breaks feature unification
no_std = []
```

## Notes
If two features truly cannot coexist, detect the conflict with `compile_error!` when both are set and document the limitation; do not rely on Cargo to keep them exclusive. Use `dep:` syntax to keep optional dependency names out of the feature namespace.
