---
id: obs-instrument-spans
title: Use #[tracing::instrument] and spans to attach context to async tasks and requests
category: observability
severity: warn
tier: 2
enforceability: check
tags: [tracing, spans, async]
applies_to:
  - "span.enter() held across an .await point"
  - "async request or job handlers"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['\.enter\(\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-instrument-spans
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Instrument async operations with `#[tracing::instrument]` or `.instrument(span)`, and never hold a span entry guard across an `.await`.

## Why
A span groups all events emitted during a logical operation and attaches context to each, letting you correlate log lines from concurrent async tasks. Holding a span entry guard (`let _g = span.enter()`) across an `.await` attaches the span to the wrong task when the executor resumes on another thread. `.instrument(span)` on the future, or `#[instrument]` on the function, keeps the span attached correctly.

## Do
```rust
use tracing::{info, instrument, Instrument, info_span};

#[instrument(skip(db), fields(user.id = user_id))]
async fn fetch_user(user_id: u64, db: &DbPool) -> Result<String, DbError> {
    info!("fetching user from database");
    let user = db.query_user(user_id).await?;
    info!(username = %user.name, "user fetched");
    Ok(user.name)
}

async fn process_job(job_id: &str) {
    let span = info_span!("process_job", job.id = job_id);
    async move {
        info!("job started");
        do_work().await;
        info!("job complete");
    }
    .instrument(span)
    .await;
}

async fn do_work() {}

struct DbPool;
#[derive(Debug)] struct DbUser { name: String }
#[derive(Debug)] struct DbError;

impl DbPool {
    async fn query_user(&self, _id: u64) -> Result<DbUser, DbError> {
        Ok(DbUser { name: "alice".to_string() })
    }
}
```

## Don't
```rust
use tracing::{info, span, Level};

async fn fetch_user(user_id: u64) -> Result<String, String> {
    let span = span!(Level::INFO, "fetch_user", user_id);
    let _guard = span.enter();

    let result = some_async_db_call(user_id).await;
    info!("fetched user");
    result
}

async fn some_async_db_call(_id: u64) -> Result<String, String> {
    Ok("alice".to_string())
}
```

## Notes
Use `skip(arg)`/`skip_all` on `#[instrument]` to exclude large types (pools, byte buffers) and sensitive values from auto-captured fields, and `fields(key = value)` to add or rename fields. Holding a guard across `.await` in a fully synchronous span (no await inside) is fine.
