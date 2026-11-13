---
id: api-builder-must-use
title: Mark builder methods with #[must_use] so ignored return values warn
category: api-design
severity: warn
tier: 2
enforceability: gate
tags: [builder, must-use, ergonomics]
applies_to:
  - "fn that takes self and returns Self"
  - "builder methods returning the builder"
rust_edition: "2024"
msrv: null
detect:
  clippy: [return_self_not_must_use]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-builder-must-use
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Annotate builder methods (and the builder type) with `#[must_use]` so that calling one and discarding the returned builder produces a compiler warning.

## Why
Consuming builder methods return a modified builder by value. If a caller ignores that return value, the builder is dropped and the configuration is silently lost, producing code that looks correct but has no effect. `#[must_use]` turns that silent drop into a warning, catching the mistake at compile time.

## Do
```rust
use std::time::Duration;

struct RequestBuilder {
    timeout: Option<Duration>,
}

impl RequestBuilder {
    #[must_use = "builder methods return the modified builder - chain or assign"]
    fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }
}
```

## Don't
```rust
use std::time::Duration;

struct RequestBuilder {
    timeout: Option<Duration>,
}

impl RequestBuilder {
    fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }
}

let request = RequestBuilder { timeout: None };
request.timeout(Duration::from_secs(30));
```
