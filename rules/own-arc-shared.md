---
id: own-arc-shared
title: Use Arc<T> for shared ownership that crosses thread boundaries
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [concurrency, shared-ownership, atomics]
applies_to:
  - "value shared by reference across std::thread::spawn or tokio::spawn"
  - "Rc<T> that must be Send"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-arc-shared
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When data must be shared by ownership across threads, wrap it in `Arc<T>`; pair it with `Mutex`/`RwLock` only when the shared data must also be mutated.

## Why
`Arc` updates its reference count atomically, so cloning a handle is safe from any thread, and the type is `Send + Sync` when `T` is. `Rc` uses non-atomic counts and is `!Send`, so the compiler rejects it at a thread boundary. Reaching for `Arc` where `Rc` suffices pays atomic overhead for nothing, so scope it to the threads-actually-cross case.

## Do
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let handle = {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        println!("{data:?}");
    })
};
handle.join().unwrap();
println!("{data:?}");
```

## Don't
```rust
use std::rc::Rc;
use std::thread;

let data = Rc::new(vec![1, 2, 3]);
let data = Rc::clone(&data);
thread::spawn(move || {
    println!("{data:?}");
});
```

## Notes
Prefer `std::thread::scope` to borrow stack data directly when the threads are joined before the borrow ends — it avoids `Arc` entirely. Use `Rc` in confirmed single-threaded code where the atomic overhead is measurable.
