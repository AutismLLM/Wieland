---
id: api-builder-pattern
title: Use the builder pattern for types with many optional construction parameters
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [builder, construction, ergonomics]
applies_to:
  - "constructor with many parameters"
  - "type with many optional fields"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-builder-pattern
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a type has many optional parameters or complex initialization, expose a builder instead of a single constructor with a long positional argument list.

## Why
Constructors with many positional arguments are error-prone: callers can transpose arguments of the same type, and the call site does not say what each value means. A builder makes each option a named, chainable method, so construction is self-documenting and callers only set what they need. `build()` centralizes validation and defaulting.

## Do
```rust
use std::time::Duration;

#[derive(Default)]
#[must_use = "builders do nothing unless you call build()"]
pub struct ClientBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: u32,
}

impl ClientBuilder {
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
}

let builder = ClientBuilder::default()
    .base_url("https://api.example.com")
    .timeout(Duration::from_secs(10))
    .max_retries(5);
```

## Don't
```rust
use std::time::Duration;

fn make_client(
    base_url: &str,
    timeout: Duration,
    retries: u32,
    verbose: bool,
    follow_redirects: bool,
    auth_token: Option<&str>,
) {
}

make_client("https://api.example.com", Duration::from_secs(10), 5, true, false, None);
```
