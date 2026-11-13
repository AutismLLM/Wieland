---
id: async-mpsc-queue
title: Use tokio mpsc for task-to-task queues, never std sync mpsc in async code
category: async
severity: warn
tier: 2
enforceability: check
tags: [channels, blocking, mpsc]
applies_to:
  - "std::sync::mpsc used inside async tasks"
  - "message queue between spawned tasks"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['std::sync::mpsc']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-mpsc-queue
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For message passing between async tasks use `tokio::sync::mpsc`; never call blocking `std::sync::mpsc` inside an async task.

## Why
`std::sync::mpsc::Receiver::recv` blocks the OS thread until a message arrives, which parks an entire executor worker and starves every other task scheduled on it. `tokio::sync::mpsc` provides async `send`/`recv` that yield instead of blocking, cheap sender cloning for multiple producers, and bounded capacity for backpressure — the default channel for task-to-task communication.

## Do
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<String>(100);

tokio::spawn(async move {
    tx.send("hello".to_string()).await.unwrap();
});

tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {
        println!("received: {msg}");
    }
});
```

## Don't
```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

tokio::spawn(async move {
    tx.send("hello").unwrap();
});

tokio::spawn(async move {
    let msg = rx.recv().unwrap();
    println!("received: {msg}");
});
```

## Notes
Drop or `downgrade` surplus senders so the receive loop ends when the last strong sender goes away. Pair with a `oneshot` reply channel for request-response actors.
