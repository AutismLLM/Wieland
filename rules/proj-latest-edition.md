---
id: proj-latest-edition
title: Target the latest stable Rust edition for new crates
category: project-layout
severity: warn
tier: 2
enforceability: check
tags: [cargo, edition, layout]
applies_to:
  - "Cargo.toml [package] section of a new crate"
  - "a new workspace root Cargo.toml"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['edition\s*=\s*"2015"', 'edition\s*=\s*"2018"', 'edition\s*=\s*"2021"']
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-LATEST-EDITION
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When starting a new crate or workspace, set `edition` to the latest stable edition (`2024` as of this writing). There is no ecosystem-compatibility reason to pick an older one.

## Do
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2024"
```

## Don't
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"  # no compatibility benefit, just older idioms
```

## Why
Downstream compatibility is governed by MSRV, not edition — a `2015`-edition binary can depend on `2024`-edition libraries without friction. Picking an old edition for a new crate only forces you to write against older defaults (e.g. weaker `dyn` and closure-capture ergonomics) with zero upside. The `resolver` field does not need to be set separately; it follows from the edition.

## Notes
This is about the edition of newly created crates, not about churning the edition of an existing one — an edition bump on an established crate is a separate, deliberate migration (`cargo fix --edition`), not something to do reflexively.
