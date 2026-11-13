---
id: own-rwlock-readers
title: Use RwLock<T> when reads significantly outnumber writes
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [concurrency, rwlock, read-heavy]
applies_to:
  - "config or cache read by many threads, written rarely"
  - "Mutex serializing read-only access"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-rwlock-readers
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For read-heavy shared state, use `RwLock<T>` so many readers can proceed concurrently, reserving exclusive access for the occasional writer.

## Why
`Mutex` serializes every access, including reads, so a config read by a hundred threads is handled one at a time. `RwLock` grants any number of concurrent `read` guards or a single exclusive `write` guard, so read-dominated workloads gain real parallelism. The reader bookkeeping is not free, though — when writes are frequent or locks are held only briefly, a plain `Mutex` is faster.

## Do
```rust
use std::sync::{Arc, RwLock};

let config = Arc::new(RwLock::new(Config::load()));

fn get_setting(config: &RwLock<Config>, key: &str) -> String {
    config.read().unwrap().get(key).to_string()
}

fn update_setting(config: &RwLock<Config>, key: &str, value: &str) {
    config.write().unwrap().set(key, value);
}
```

## Don't
```rust
use std::sync::{Arc, Mutex};

let config = Arc::new(Mutex::new(Config::load()));

fn get_setting(config: &Mutex<Config>, key: &str) -> String {
    config.lock().unwrap().get(key).to_string()
}
```

## Notes
Std `RwLock` can starve writers under continuous reads; `parking_lot::RwLock` is writer-fair and adds an upgradable read guard. Prefer `Mutex` when writes exceed roughly 20% of operations.
