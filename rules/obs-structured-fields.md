---
id: obs-structured-fields
title: Record structured key-value fields, not values interpolated into the message string
category: observability
severity: warn
tier: 2
enforceability: check
tags: [tracing, structured, fields]
applies_to:
  - "tracing event macros with values interpolated into the message"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['(info|warn|error|debug|trace)!\("[^"]*\{[^"]*\}']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-structured-fields
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Attach values to log events as discrete `key = value` fields and keep the message string a short, stable description.

## Why
When values are interpolated into the message (e.g., `"processed 42 items for user 7"`), they become opaque text. Aggregators like Loki, Elasticsearch, and OpenTelemetry cannot filter on `items = 42` or group by `user.id` because those values no longer exist as fields. Structured fields stay machine-parseable, filterable, and chart-able without regex post-processing.

## Do
```rust
use tracing::info;

fn process_batch(user_id: u64, items: usize, elapsed_ms: u64) {
    info!(
        user.id = user_id,
        items,
        elapsed_ms,
        "batch processed"
    );
}
```

## Don't
```rust
use tracing::info;

fn process_batch(user_id: u64, items: usize, elapsed_ms: u64) {
    info!("processed {} items for user {} in {}ms", items, user_id, elapsed_ms);
}
```

## Notes
Field sigils: bare `field = value` records typed primitives, `%expr` uses `Display`, `?expr` uses `Debug`. Prefer `%` over `?` for values with a clean `Display`, and use namespaced names like `http.status` to align with OpenTelemetry conventions.
