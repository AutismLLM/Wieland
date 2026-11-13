---
id: proj-split-crate-boundary
title: Split an independently usable module out into its own crate
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [cargo, crates, compile-time, modularity]
applies_to:
  - "a submodule usable on its own outside the current crate"
  - "growing compile times in a single large crate"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SMALLER-CRATES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When in doubt, split: if a module could stand on its own, move it into its own crate rather than leaving it nested inside a larger one.

## Why
Crate boundaries are Cargo's unit of incremental compilation and parallelism — more, smaller crates compile faster during iteration and cannot form cyclic dependencies. A submodule usable independently gains nothing from staying nested except avoiding the split's paperwork, while costing every downstream consumer a full rebuild of unrelated code whenever anything in the mega-crate changes.

## Do
```toml
# workspace root
[workspace]
members = ["crates/web_server", "crates/web_client", "crates/web_protocols"]
```
```rust
// crates/web_protocols/src/lib.rs
pub struct Request;
pub struct Response;
```
```rust
// crates/web_client/src/lib.rs
use web_protocols::{Request, Response};

pub fn send(_req: Request) -> Response {
    Response
}
```

## Don't
```rust
// one crate, one lib.rs — client-only users still compile the server
pub mod server {
    pub fn run() {}
}
pub mod client {
    pub fn send() {}
}
pub mod protocols {
    pub struct Request;
    pub struct Response;
}
```

## Notes
Splitting a crate can cost you `pub(crate)` visibility across the new boundary — treat that friction as a signal to design a proper public API, not a reason to stay merged. Where a split is purely technical (a proc-macro crate, a generated-code crate), re-export it from an umbrella crate so users still get one dependency to add.
