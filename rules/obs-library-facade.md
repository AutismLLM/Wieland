---
id: obs-library-facade
title: Emit through the tracing/log facade in a library; install the subscriber only in main
category: observability
severity: warn
tier: 2
enforceability: check
tags: [tracing, library, subscriber]
applies_to:
  - "tracing_subscriber::fmt::init in a library crate"
  - "env_logger::init in a library crate"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['tracing_subscriber::fmt(::fmt)?::(try_)?init', 'env_logger::(try_)?init']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-library-facade
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A library crate only emits `tracing`/`log` events and spans; the binary that owns `main` installs the single global subscriber.

## Why
Installing a global subscriber or logger is a one-time, process-wide operation. If a library calls `tracing_subscriber::fmt::init()` or `env_logger::init()`, it conflicts with any other library or the application that does the same — the second call panics or is silently ignored, and the caller loses control over log format, destination, and level filtering. Libraries emit; the application decides how to handle.

## Do
```rust
use tracing::info;

pub fn connect(url: &str) {
    info!(url, "connecting");
}
```

## Don't
```rust
use tracing::info;

pub fn connect(url: &str) {
    tracing_subscriber::fmt::init();
    info!(url, "connecting");
}
```

## Notes
A library's `Cargo.toml` should depend on `tracing = "0.1"` only; keep `tracing-subscriber`/`env_logger` in `[dev-dependencies]` for tests, and initialize inside `#[test]` functions with the non-panicking `tracing_subscriber::fmt::try_init()`.
