---
id: doc-magic-value-rationale
title: Document why a magic value was chosen, not just what it is
category: docs
severity: warn
tier: 1
enforceability: judge
tags: [docs, constants, maintainability]
applies_to:
  - "hardcoded numeric or string literals in production code paths"
  - "named const/static items with non-obvious values"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-DOCUMENTED-MAGIC
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give every magic value a named constant, and document on that constant why the value was chosen, what breaks if it changes, and which external system it is tied to.

## Why
A bare literal like `60 * 60 * 24` tells a future editor *what* the value is but not *why* it is safe to change or what depends on it. A comment that only restates the value ("wait one day") is nearly as useless. The value that actually prevents a bad refactor is the rationale: why this number, what happens if it moves, and which upstream contract pins it in place.

## Do
```rust
use std::time::Duration;

/// How long we wait for the upstream server to respond.
///
/// Large enough to cover the server's worst-case processing time; setting
/// this too low aborts otherwise-valid requests. Derived from `api.foo.com`'s
/// documented timeout policy (24h max processing window).
const UPSTREAM_SERVER_TIMEOUT: Duration = Duration::from_secs(60 * 60 * 24);

async fn call_upstream() {
    wait_timeout(UPSTREAM_SERVER_TIMEOUT).await;
}

async fn wait_timeout(_d: Duration) {}
```

## Don't
```rust
async fn call_upstream() {
    // waits for a day
    wait_timeout(60 * 60 * 24).await;
}

async fn wait_timeout(_secs: u64) {}
```

## Notes
Not every literal needs this treatment — `0`, `1`, and array lengths obvious from context don't qualify as "magic." Reserve the rationale comment for values that encode a business rule, a third-party contract, or a tuning decision someone could plausibly get wrong when changing later.
