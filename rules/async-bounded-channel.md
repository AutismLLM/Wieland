---
id: async-bounded-channel
title: Use bounded channels to apply backpressure and bound memory
category: async
severity: warn
tier: 2
enforceability: check
tags: [channels, backpressure, memory]
applies_to:
  - "tokio::sync::mpsc::unbounded_channel"
  - "producer/consumer task pair"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['unbounded_channel']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-bounded-channel
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Prefer a bounded `mpsc::channel(cap)` over `unbounded_channel` so a fast producer is throttled by backpressure instead of exhausting memory.

## Why
An unbounded channel grows without limit whenever producers outpace consumers, and in production that ends in OOM. A bounded channel makes the sender wait once the buffer is full, throttling the whole pipeline naturally and keeping memory usage predictable. Capacity becomes an explicit, tunable resource limit rather than an accident.

## Do
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<Message>(100);

tokio::spawn(async move {
    loop {
        let msg = generate_message();
        tx.send(msg).await.unwrap();
    }
});

tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {
        slow_process(msg).await;
    }
});
```

## Don't
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

tokio::spawn(async move {
    loop {
        let msg = generate_message();
        tx.send(msg).unwrap();
    }
});

tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {
        slow_process(msg).await;
    }
});
```

## Notes
When you must not block a synchronous or latency-critical producer, use `try_send` and handle `TrySendError::Full` explicitly (drop, shed, or measure) rather than reaching for an unbounded channel.
