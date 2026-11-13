---
id: async-watch-latest
title: Use a watch channel to share the latest value with many observers
category: async
severity: info
tier: 1
enforceability: judge
tags: [channels, watch, state]
applies_to:
  - "config/state/status shared to multiple readers"
  - "receivers that only need the newest value"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-watch-latest
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When observers only need the most recent value and may skip intermediate ones, share it through a `tokio::sync::watch` channel.

## Why
`watch` holds a single current value that every receiver can `borrow` immediately and `await` for changes. A slow receiver simply skips to the latest value instead of lagging (`broadcast`) or applying backpressure (`mpsc`), which is exactly right for configuration, status, or state that should always reflect the present. It avoids processing stale intermediate values entirely.

## Do
```rust
use tokio::sync::watch;

let (tx, rx) = watch::channel(Config::default());

let mut rx1 = rx.clone();
tokio::spawn(async move {
    while rx1.changed().await.is_ok() {
        let config = rx1.borrow().clone();
        apply_config(&config);
    }
});

tx.send(Config::new()).unwrap();
```

## Don't
```rust
use tokio::sync::broadcast;

let (tx, mut rx) = broadcast::channel::<Config>(100);

tokio::spawn(async move {
    while let Ok(config) = rx.recv().await {
        apply_config(&config);
    }
});
```

## Notes
`borrow()` returns a `Ref` that must not be held across an `.await` — clone the value out, or use `borrow_and_update()` to also mark it seen. Use `send_if_modified` to notify observers only when the value actually changed.
