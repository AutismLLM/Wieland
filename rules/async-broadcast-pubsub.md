---
id: async-broadcast-pubsub
title: Use a broadcast channel when every subscriber must receive every message
category: async
severity: info
tier: 1
enforceability: judge
tags: [channels, pubsub, fan-out]
applies_to:
  - "event bus / notification fan-out"
  - "multiple independent consumers of the same events"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-broadcast-pubsub
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For pub/sub fan-out where each subscriber must see every event, use `tokio::sync::broadcast` rather than trying to share one `mpsc` receiver.

## Why
`mpsc` delivers each message to exactly one consumer, so it cannot fan the same event out to several independent reactors. `broadcast` clones each message to every subscriber created with `subscribe()`, which is what event buses, notifications, and multi-listener systems actually need. Slow subscribers surface as `RecvError::Lagged` so drops are observable rather than silent.

## Do
```rust
use tokio::sync::broadcast;

let (tx, _) = broadcast::channel::<Event>(100);

let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    while let Ok(event) = rx1.recv().await {
        handle_in_logger(event);
    }
});

tokio::spawn(async move {
    while let Ok(event) = rx2.recv().await {
        handle_in_metrics(event);
    }
});

tx.send(Event::UserLogin { user_id: 42 }).unwrap();
```

## Don't
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<Event>(100);

let logger = tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        handle_in_logger(event);
    }
});
```

## Notes
When subscribers only care about the newest value and may skip intermediate ones (config, status), use `watch` instead. `broadcast` requires the message type to be `Clone`; wrap large non-`Clone` payloads in `Arc`.
