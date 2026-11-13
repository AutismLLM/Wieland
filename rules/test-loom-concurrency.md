---
id: test-loom-concurrency
title: Verify lock-free and concurrent code with loom, not stress loops
category: testing
severity: warn
tier: 1
enforceability: judge
tags: [concurrency, loom, atomics, model-checking]
applies_to:
  - "hand-written lock-free primitives"
  - "atomic Ordering usage under test"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-loom-concurrency
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Test custom concurrent and lock-free code with `loom`, gating the synchronization primitives behind `#[cfg(loom)]` so the same logic runs under loom's model checker and under std types in production.

## Why
A probabilistic stress test can execute millions of iterations and still never hit the one thread interleaving that exposes a race, because the OS scheduler decides the ordering. `loom` systematically explores every interleaving and memory reordering the C11 model permits, turning "it seemed fine" into coverage of the interleavings within the model. Tokio uses loom to verify its own primitives.

## Do
```rust
#[cfg(loom)]
use loom::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(loom))]
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Flag(AtomicBool);

impl Flag {
    pub fn new() -> Self {
        Self(AtomicBool::new(false))
    }
    pub fn set(&self) {
        self.0.store(true, Ordering::Release);
    }
    pub fn is_set(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
```

## Don't
```rust
#[test]
fn stress_test_flag() {
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    let flag = Arc::new(AtomicBool::new(false));
    for _ in 0..1_000_000 {
        let flag = Arc::clone(&flag);
        std::thread::spawn(move || {
            flag.store(true, Ordering::Relaxed);
        });
    }
}
```

## Notes
Keep each `loom::model(|| { ... })` closure small — the state space grows combinatorially with the number of atomics and threads. loom checks the memory model only; it will not catch logic bugs unrelated to concurrency.
