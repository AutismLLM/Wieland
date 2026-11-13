---
id: async-spawn-blocking
title: Move CPU-heavy or blocking work off the async runtime with spawn_blocking
category: async
severity: warn
tier: 1
enforceability: judge
tags: [blocking, cpu-bound, runtime]
applies_to:
  - "CPU-intensive computation inside an async fn"
  - "synchronous std::fs / blocking library call in async code"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-spawn-blocking
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Offload CPU-intensive or unavoidably blocking work to `tokio::task::spawn_blocking` (or a proper async API) instead of running it directly on an executor thread.

## Why
A Tokio runtime services many tasks from a small pool of worker threads. A long computation or blocking syscall on one of those threads stalls every other task scheduled there and can deadlock the runtime. `spawn_blocking` moves the work to a dedicated blocking pool, keeping the async workers free to make progress.

## Do
```rust
use tokio::task;

async fn hash_password(password: String) -> String {
    task::spawn_blocking(move || {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
    })
    .await
    .unwrap()
}
```

## Don't
```rust
async fn hash_password(password: String) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}
```

## Notes
Rough guide: under ~10µs is fine on the async thread, over ~1ms belongs on `spawn_blocking`. For true parallel CPU work, run Rayon inside a single `spawn_blocking`. Prefer a real async API (`tokio::fs`, an async DB driver) over `spawn_blocking` when one exists.
