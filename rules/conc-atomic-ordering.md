---
id: conc-atomic-ordering
title: Use the weakest correct memory Ordering for every atomic operation
category: concurrency
severity: info
tier: 1
enforceability: judge
tags: [atomics, memory-model, correctness]
applies_to:
  - "AtomicU64/AtomicBool/AtomicUsize operations"
  - "Ordering::SeqCst used by default"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#conc-atomic-ordering
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Pick the weakest memory `Ordering` that is still correct for each atomic operation instead of reaching for `SeqCst` everywhere.

## Why
Defaulting to `SeqCst` is correct but not free: on ARM and RISC-V it lowers to full memory barriers where `Relaxed`, `Acquire`, and `Release` map to cheaper instructions. More importantly, ordering is a correctness property the compiler cannot check for you — an independent counter wants `Relaxed`, a producer/consumer handoff wants `Release`/`Acquire`, and only cross-atomic total order needs `SeqCst`. Choosing deliberately is both faster and clearer about intent.

## Do
```rust
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn increment() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

static READY: AtomicBool = AtomicBool::new(false);
static VALUE: AtomicU64 = AtomicU64::new(0);

fn producer(value: u64) {
    VALUE.store(value, Ordering::Relaxed);
    READY.store(true, Ordering::Release);
}

fn consumer() -> Option<u64> {
    if READY.load(Ordering::Acquire) {
        Some(VALUE.load(Ordering::Relaxed))
    } else {
        None
    }
}
```

## Don't
```rust
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn producer() {
    READY.store(true, Ordering::SeqCst);
}
```

## Notes
When the interaction genuinely requires a single total order observed by all threads across multiple atomic variables (Dekker-style mutual exclusion), `SeqCst` is the correct choice — do not weaken it there. Verify non-obvious ordering choices with the `loom` crate.
