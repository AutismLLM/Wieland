---
id: anti-lock-across-await
title: Never hold a lock guard across an await point
category: anti-patterns
severity: deny
tier: 3
enforceability: gate
tags: [async, concurrency, deadlock]
applies_to:
  - "a Mutex/RwLock guard still live across an .await"
  - "RefCell borrow held across an .await"
rust_edition: "2024"
msrv: null
detect:
  clippy: [await_holding_lock, await_holding_refcell_ref]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-lock-across-await
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Release a `Mutex`/`RwLock` guard before any `.await`; scope the lock, extract or clone what you need, then await.

## Why
A guard held across `.await` keeps the lock while the task is suspended, so other tasks blocking on it stall indefinitely. With `std::sync::Mutex` this can deadlock the whole runtime, because the guard is not designed to be held across suspension points. Even `tokio::sync::Mutex` merely blocks other tasks for the full duration of the awaited work.

## Do
```rust
use std::sync::Mutex;

async fn append(data: &Mutex<Vec<i32>>) {
    let value = {
        let guard = data.lock().unwrap();
        guard.last().copied()
    };
    let result = do_async_work(value).await;
    data.lock().unwrap().push(result);
}
```

## Don't
```rust
use std::sync::Mutex;

async fn append(data: &Mutex<Vec<i32>>) {
    let mut guard = data.lock().unwrap();
    do_async_work().await;
    guard.push(42);
}
```

## Notes
When shared state is contended, consider restructuring to owned data behind a channel (`mpsc` + `oneshot`) instead of a shared lock, so no guard ever needs to survive an await.
