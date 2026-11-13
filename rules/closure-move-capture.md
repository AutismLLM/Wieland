---
id: closure-move-capture
title: Use move for closures that outlive the scope; clone before move to keep the original
category: closures
severity: info
tier: 1
enforceability: judge
tags: [move, threads, ownership, lifetimes]
applies_to:
  - "closure passed to thread::spawn"
  - "closure returned from a function"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#closure-move-capture
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Add `move` to a closure that escapes its scope so it owns its captures, and clone the value first when you also need it in the surrounding code.

## Why
A closure that borrows its environment can only live as long as that environment. When it escapes — spawned on a thread, stored in a struct, or returned — it must own its captures and is usually required to be `'static`. `move` transfers ownership of every captured variable into the closure; if the surrounding code still needs a value, clone it and move the clone rather than the original.

## Do
```rust
fn process(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn spawn_and_keep(data: Vec<i32>) -> std::thread::JoinHandle<i32> {
    let data_for_thread = data.clone();
    let handle = std::thread::spawn(move || process(&data_for_thread));
    println!("original still owned: {data:?}");
    handle
}

fn demo() {
    let nums = vec![10, 20, 30];
    let handle = spawn_and_keep(nums);
    assert_eq!(handle.join().unwrap(), 60);
}
```

## Don't
```rust
fn spawn_bad() {
    let data = vec![1, 2, 3];
    std::thread::spawn(|| println!("{data:?}"));
}
```

## Notes
Async tasks follow the same rule: `tokio::spawn(async move { … })` takes ownership, so clone shared data before the `async move` block. Clone only what the closure needs, not the whole owning struct. `move` controls how captures are taken, not how many times the closure can be called — a `move` closure can still implement `Fn`.
