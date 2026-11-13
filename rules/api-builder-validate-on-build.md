---
id: api-builder-validate-on-build
title: Keep builder setters infallible and validate only in the final build()
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [builder, validation, error-handling]
applies_to:
  - "builder setter method returning Result"
  - "builder type with a fallible build() method"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-BUILD-RESULT
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Builder setters should accept their input and return `Self` without failing. Push all validation, including checks that span multiple fields, into a single fallible `build()`.

## Why
A setter that returns `Result` forces the caller to handle (or unwrap) an error after every single call, even though most individual fields cannot be checked in isolation and the builder cannot be used until `build()` anyway. That per-field noise buys nothing: it still cannot catch invariants that depend on two fields together. Consolidating all validation in `build()` gives one place to check cross-field conditions and one `Result` for the caller to handle.

## Do
```rust
#[derive(Default)]
pub struct FooBuilder {
    name: Option<String>,
    distance: Option<u32>,
}

impl FooBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn distance(mut self, distance: u32) -> Self {
        self.distance = Some(distance);
        self
    }

    pub fn build(self) -> Result<Foo, FooBuildError> {
        let name = self.name.ok_or(FooBuildError::MissingName)?;
        let distance = self.distance.ok_or(FooBuildError::MissingDistance)?;
        Ok(Foo { name, distance })
    }
}

let foo = FooBuilder::default().name("Foo").distance(42).build()?;
```

## Don't
```rust
#[derive(Default)]
pub struct FooBuilder {
    name: Option<String>,
    distance: Option<u32>,
}

impl FooBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Result<Self, FooBuildError> {
        self.name = Some(name.into());
        Ok(self)
    }

    pub fn distance(mut self, distance: u32) -> Result<Self, FooBuildError> {
        self.distance = Some(distance);
        Ok(self)
    }
}

let foo = FooBuilder::default().name("Foo")?.distance(42)?;
```

## Notes
Where an individual setting has its own validation rule (a port number, a non-empty name), prefer accepting an already-validated newtype (see `type-newtype-validated`) so the setter itself stays infallible and the check happens once, at the newtype's own constructor.
