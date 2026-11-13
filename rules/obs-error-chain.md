---
id: obs-error-chain
title: Log errors with their full source chain, and log each error exactly once
category: observability
severity: warn
tier: 1
enforceability: judge
tags: [tracing, errors, source-chain]
applies_to:
  - "logging an error inside a map_err before propagating with ?"
  - "logging the same error at multiple layers"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-error-chain
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Log an error once, at the layer that handles it, capturing its full source chain; propagate everywhere else with `?` and added context.

## Why
Logging only an error's top-level `Display` drops the underlying cause chain — you see "request failed" but not why. Logging at every propagation layer records the same failure repeatedly with differing context, polluting aggregators. Capture the chain with `error = ?err` (Debug) or `{:#}` on an `anyhow::Error`, and emit it once at the handling boundary.

## Do
```rust
use anyhow::{Context, Result};
use tracing::{error, instrument};

#[instrument]
async fn read_from_db(id: u64) -> Result<Vec<u8>> {
    inner_db_call(id)
        .await
        .with_context(|| format!("failed to read record {id} from database"))
}

#[instrument]
async fn fetch_data(id: u64) -> Result<Vec<u8>> {
    read_from_db(id).await.context("fetch_data failed")
}

#[instrument]
async fn handle_request(id: u64) -> Result<(), String> {
    match fetch_data(id).await {
        Ok(data) => {
            process(data);
            Ok(())
        }
        Err(err) => {
            error!(error = %format!("{err:#}"), "request failed");
            Err("internal error".to_string())
        }
    }
}

async fn inner_db_call(_id: u64) -> Result<Vec<u8>> {
    Err(anyhow::anyhow!("connection refused"))
}
fn process(_data: Vec<u8>) {}
```

## Don't
```rust
use tracing::error;

async fn fetch_data(id: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = read_from_db(id).await.map_err(|e| {
        error!("{}", e);
        e
    })?;
    Ok(data)
}

async fn handle(id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data(id).await.map_err(|e| {
        error!("{}", e);
        e
    })?;
    process(data);
    Ok(())
}

async fn read_from_db(_id: u64) -> Result<Vec<u8>, std::io::Error> {
    Err(std::io::Error::other("connection refused"))
}
fn process(_data: Vec<u8>) {}
```

## Notes
If you must log at a non-handling layer (e.g., a background task that discards the error), use `warn!` to signal it was absorbed. The `tracing-error` crate's `SpanTrace` captures span context at the error site and attaches it to the error type.
