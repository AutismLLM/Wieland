---
id: anti-static-correctness-state
title: Never hold correctness-relevant state in a static or thread-local
category: anti-patterns
severity: warn
tier: 1
enforceability: judge
tags: [globals, statics, testability, semver]
applies_to:
  - "static item whose value affects program correctness"
  - "library crate declaring module-level mutable global state"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-AVOID-STATICS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A library must not keep state in a `static` or thread-local when a caller relying on a particular value would be wrong if that value secretly diverged. Reserve statics for read-only constants or pure performance caches; thread anything correctness-relevant through instances instead.

## Why
Cargo can link multiple incompatible versions of the same crate into one binary, especially during a crate's `0.x` line where every minor bump is a semver break. Each linked version gets its own copy of every `static`, so two call sites that "obviously" share one counter, cache, or registry can silently observe different values depending on which version of the crate resolved for each caller. Statics also make unit tests interfere with each other (shared mutable state across `#[test]` functions) and become a contention point in thread-per-core designs. If the crate would still be correct with the static's value replaced by another instance's value, it is a performance cache and is fine; if not, it must live on an instance the caller owns.

## Do
```rust
pub struct RequestCounter {
    count: std::sync::atomic::AtomicU64,
}

impl RequestCounter {
    pub fn new() -> Self {
        Self { count: std::sync::atomic::AtomicU64::new(0) }
    }

    pub fn record(&self) -> u64 {
        self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

let counter = RequestCounter::new();
counter.record();
```

## Don't
```rust
use std::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn record_request() -> u64 {
    GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed)
}
```

## Notes
A `static` backing a memoized, purely-derived value (a compiled regex, a `LazyLock` config cache reloadable from the same inputs) is fine because any duplicate copy converges to the same answer. The failure mode this rule targets is state that accumulates or is externally mutated — counters, registries, connection pools — where two copies diverge and the caller has no way to know which one they got.
