---
id: async-no-lock-await
title: Never hold a Mutex or RwLock guard across an .await point
category: async
severity: deny
tier: 0
enforceability: gate
tags: [async, correctness, performance, deadlock]
applies_to:
  - "lock().await followed by another .await"
  - "MutexGuard held across await"
  - "tokio::sync::Mutex"
rust_edition: "2024"
msrv: null
detect:
  clippy: [await_holding_lock]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-no-lock-await
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Release a `Mutex`/`RwLock` guard before any `.await`; extract the data you need, drop the guard, then await.

## Why
If a task is suspended at an `.await` while still holding a lock, every other task waiting on that lock is blocked for the entire duration of the suspended operation — potentially forever if the awaited work also needs the lock. This causes deadlocks and severe tail-latency spikes. Fetching data before taking the lock, or cloning the needed value out and dropping the guard, keeps the critical section short and non-blocking.

## Do
```rust
use tokio::sync::Mutex;

async fn good_update(state: &Mutex<State>) {
    let data = fetch_from_network().await;

    let mut guard = state.lock().await;
    guard.value = data;
}

async fn read_then_process(state: &Mutex<State>) {
    let id = {
        let guard = state.lock().await;
        guard.id.clone()
    };
    let data = fetch_by_id(id).await;
    state.lock().await.value = data;
}
```

## Don't
```rust
use tokio::sync::Mutex;

async fn bad_update(state: &Mutex<State>) {
    let mut guard = state.lock().await;
    let data = fetch_from_network().await;
    guard.value = data;
}
```

## Notes
Prefer `std::sync::Mutex` for critical sections that never span an `.await` — it is simpler and faster. Reach for `tokio::sync::Mutex` only when you genuinely must hold the lock across an await, which is rare and usually a sign to redesign around message passing or clone-out-then-update.
