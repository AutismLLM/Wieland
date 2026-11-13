---
id: obs-tracing-over-log
title: Use tracing for structured, span-aware diagnostics instead of println! or bare log
category: observability
severity: warn
tier: 2
enforceability: check
tags: [tracing, logging, diagnostics]
applies_to:
  - "println!"
  - "eprintln!"
  - "diagnostic output in library or service code"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['println!\(', 'eprintln!\(']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-tracing-over-log
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Emit diagnostics through `tracing` for levels, structured fields, and span context rather than `println!`/`eprintln!` or the bare `log` facade.

## Why
`println!` and `eprintln!` have no levels, targets, or structured data — they cannot be silenced, filtered, or parsed by observability pipelines. The `log` facade adds levels but emits only flat strings with no notion of spans. `tracing` records both events and spans (contextual scopes that follow execution across `.await` points and threads), with structured key-value fields, level filtering, and `log`-ecosystem interop via its `log` feature.

## Do
```rust
use tracing::info;

fn handle_login(id: u64) {
    info!(user.id = %id, "user logged in");
}

fn main() {
    tracing_subscriber::fmt::init();
    handle_login(42);
}
```

## Don't
```rust
fn handle_login(id: u64) {
    println!("user {} logged in", id);
}

fn main() {
    handle_login(42);
}
```

## Notes
`tracing` ships a `log` compatibility bridge: enable `tracing-subscriber`'s `log` feature or call `tracing_log::LogTracer::init()` to capture events from existing `log`-emitting dependencies. `println!` is still fine for CLI program output that is the tool's actual result, not diagnostics.
