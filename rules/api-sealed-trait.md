---
id: api-sealed-trait
title: Seal a public trait to allow its use but forbid external implementations
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [traits, sealed, public-api]
applies_to:
  - "pub trait you want to extend later"
  - "trait whose implementations you must control"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-sealed-trait
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a public trait may only be implemented by your own crate, seal it: make it a supertrait of a private marker trait so external code can call it but not implement it.

## Why
A normal public trait can be implemented by anyone, which pins its method set forever — adding a required method is a breaking change — and lets external code supply implementations you cannot vet for correctness or safety invariants. Sealing keeps the trait usable as a bound and in `impl Trait` while reserving implementation to your crate, so you can add methods in minor versions and guarantee every impl upholds its contract.

## Do
```rust
mod private {
    pub trait Sealed {}
}

pub trait Driver: private::Sealed {
    fn name(&self) -> &str;
}

pub struct Postgres;
impl private::Sealed for Postgres {}
impl Driver for Postgres {
    fn name(&self) -> &str {
        "postgres"
    }
}

fn describe(driver: &impl Driver) -> &str {
    driver.name()
}
```

## Don't
```rust
pub trait Driver {
    fn name(&self) -> &str;
}

struct ExternalDriver;
impl Driver for ExternalDriver {
    fn name(&self) -> &str {
        "external"
    }
}
```

## Notes
Do not seal a trait meant as an extension point where downstream crates are expected to provide their own implementations, nor standard-style interface traits like `Iterator`. Seal for stability and invariant guarantees; leave open for genuine extensibility.
