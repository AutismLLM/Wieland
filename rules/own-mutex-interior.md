---
id: own-mutex-interior
title: Use Mutex<T> for shared mutable state across threads
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [concurrency, mutex, interior-mutability]
applies_to:
  - "state mutated by multiple threads through a shared handle"
  - "RefCell that must be Sync"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-mutex-interior
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When several threads must mutate the same data, guard it with `Mutex<T>` (typically inside an `Arc`), not `RefCell`, which is single-threaded and `!Sync`.

## Why
`Mutex<T>` is `Send + Sync` when `T: Send` and serializes access so only one thread touches the data at a time, giving safe interior mutability across threads. `RefCell` moves borrow checking to runtime but is not thread-safe, so the compiler rejects sharing it across a thread boundary. Reach for `RwLock` instead only when reads vastly outnumber writes.

## Do
```rust
use std::sync::{Arc, Mutex};

let shared = Arc::new(Mutex::new(Vec::new()));
let handles: Vec<_> = (0..10)
    .map(|i| {
        let shared = Arc::clone(&shared);
        std::thread::spawn(move || {
            shared.lock().unwrap().push(i);
        })
    })
    .collect();
for h in handles {
    h.join().unwrap();
}
```

## Don't
```rust
use std::cell::RefCell;
use std::sync::Arc;

let shared = Arc::new(RefCell::new(Vec::new()));
std::thread::spawn({
    let shared = Arc::clone(&shared);
    move || shared.borrow_mut().push(1)
});
```

## Notes
Never hold a `Mutex` guard across an `.await` point — that can deadlock the runtime; use an async-aware lock there. `parking_lot::Mutex` is a smaller, poison-free, faster drop-in when you do not need std poisoning semantics.
