---
id: async-oneshot-response
title: Use a oneshot channel for single request-response replies
category: async
severity: info
tier: 1
enforceability: judge
tags: [channels, oneshot, request-response]
applies_to:
  - "waiting for exactly one reply to a request"
  - "actor command carrying a reply channel"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-oneshot-response
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a task sends a request and awaits exactly one response, carry the reply on a `tokio::sync::oneshot` channel rather than a buffered `mpsc` or polled shared state.

## Why
`oneshot` is a single-use channel built precisely for one value from one sender to one receiver — no capacity to size, no clone overhead, and it is consumed on receive so it cannot be accidentally reused. Reaching for `mpsc(1)` leaves a lingering channel, and shared `Arc<Mutex<Option<_>>>` forces wasteful polling. Combined with `mpsc`, `oneshot` gives clean actor-style request-response.

## Do
```rust
use tokio::sync::{mpsc, oneshot};

enum Request {
    Get { key: String, reply: oneshot::Sender<Option<Value>> },
}

async fn get_value(tx: &mpsc::Sender<Request>, key: &str) -> Option<Value> {
    let (reply_tx, reply_rx) = oneshot::channel();
    tx.send(Request::Get {
        key: key.to_string(),
        reply: reply_tx,
    })
    .await
    .ok()?;
    reply_rx.await.ok()?
}
```

## Don't
```rust
use tokio::sync::mpsc;

async fn get_value(mut rx: mpsc::Receiver<Response>) -> Response {
    send_request().await;
    rx.recv().await.unwrap()
}
```

## Notes
The sender's `send` returns the value back as `Err` if the receiver was dropped; use `is_closed`/`closed()` to skip an expensive computation when the caller has already gone away.
