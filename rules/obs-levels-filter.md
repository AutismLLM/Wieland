---
id: obs-levels-filter
title: Use log levels meaningfully and filter with EnvFilter / RUST_LOG
category: observability
severity: info
tier: 1
enforceability: judge
tags: [tracing, levels, env-filter]
applies_to:
  - "choice of log level for an event"
  - "subscriber verbosity configuration"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-levels-filter
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Pick log levels by urgency (`error`/`warn`/`info`/`debug`/`trace`) and let operators tune verbosity at runtime with `EnvFilter` and `RUST_LOG`.

## Why
Levels communicate urgency and let operators change verbosity without recompiling. Emitting everything at `info!`, or leaving `debug!` output in hot paths in production, overwhelms aggregators and hides real signals. `EnvFilter` reads `RUST_LOG` and supports per-crate, per-target, and per-span directives; tracing's `max_level_*` Cargo features can compile out verbose levels entirely in release.

## Do
```rust
use tracing::{debug, error, info, instrument, trace, warn};

#[instrument(skip(body))]
fn handle_request(path: &str, body: &[u8]) {
    trace!("entered handler");
    debug!(body_len = body.len(), "parsing body");
    info!(path, "request received");

    match parse_body(body) {
        Ok(parsed) => info!(items = parsed.len(), "request processed"),
        Err(e) if is_client_error(&e) => warn!(error = ?e, "malformed request from client"),
        Err(e) => error!(error = ?e, "unexpected parse failure"),
    }
}

fn parse_body(_body: &[u8]) -> Result<Vec<u8>, String> { Ok(vec![]) }
fn is_client_error(_e: &str) -> bool { false }
```

## Don't
```rust
use tracing::info;

fn handle_request(path: &str, body: &[u8]) {
    info!(path, body_len = body.len(), raw = ?body, "handling request");
    info!("entered handle_request");
    info!("about to parse body");
    info!("done handling request");
}
```

## Notes
Configure the subscriber with `EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,myapp=debug,hyper=warn".into())` so the binary still starts when `RUST_LOG` is unset or malformed. Compile out verbose levels in release with `features = ["release_max_level_info"]`.
