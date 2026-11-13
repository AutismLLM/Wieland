---
id: proj-test-util-feature
title: Gate mocking and test-only backdoors behind a test-util feature
category: project-layout
severity: warn
tier: 1
enforceability: judge
tags: [cargo, features, testing, security]
applies_to:
  - "public method that bypasses a safety or security check"
  - "mocking or fault-injection API exposed unconditionally"
  - "inspection method exposing otherwise-private internal state"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-TEST-UTIL
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Anything that exists only to make testing possible — mock controllers, safety-check bypasses, fake data generators, internal-state inspectors — must live behind a Cargo feature flag, conventionally named `test-util`, so it cannot ship enabled in an ordinary production build.

## Why
If a mocking hook or a "skip certificate validation" escape hatch is always compiled in, any dependent crate can call it in production, whether by accident or by a supply-chain compromise reaching for the path of least resistance. A dedicated feature flag makes the capability opt-in and visible in `Cargo.toml`, and keeps it out of the default dependency graph and the shipped binary unless a caller deliberately turns it on.

## Do
```rust
pub struct HttpClient {
    verify_certs: bool,
}

impl HttpClient {
    pub fn get(&self, url: &str) -> Result<Response, Error> {
        todo!()
    }

    #[cfg(feature = "test-util")]
    pub fn bypass_certificate_checks(&mut self) {
        self.verify_certs = false;
    }
}
```
```toml
[features]
test-util = []
```

## Don't
```rust
pub struct HttpClient {
    verify_certs: bool,
}

impl HttpClient {
    pub fn get(&self, url: &str) -> Result<Response, Error> {
        todo!()
    }

    pub fn bypass_certificate_checks(&mut self) {
        self.verify_certs = false;
    }
}
```

## Notes
Use a single, clearly named feature (`test-util`) rather than one flag per capability, so downstream users only have one opt-in to reason about. This composes with `test-mock-traits`: the trait-based seam stays in production code, while the concrete mock implementation and any bypass hooks sit behind the feature.
