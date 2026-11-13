---
id: err-catch-unwind-restart
title: Treat catch_unwind as last resort and restart after it, never resume freely
category: errors
severity: warn
tier: 2
enforceability: check
tags: [panic, error-handling, resilience]
applies_to:
  - "std::panic::catch_unwind"
  - "per-request or per-task panic isolation"
  - "long-running server processes"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['catch_unwind\(']
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-PANIC-CONTINUATION
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `catch_unwind` only as a last-resort isolation boundary — such as per-request handling in a server — and drive the process toward a controlled restart afterward rather than assuming execution can safely continue.

## Why
A panic means the invariants of everything reachable from the unwound frame are no longer trustworthy: a `thread_local`, a lock, or shared state can be left in a torn state that looks valid but isn't. `catch_unwind` stops the unwind from propagating, but it does not repair whatever the panicking code left behind. In a server handling many independent requests, catching per-request lets other in-flight work finish gracefully, but the process should still schedule a restart rather than treat the panic as a handled error and keep serving indefinitely on the same state.

## Do
```rust
use std::panic::{self, AssertUnwindSafe};

async fn handle_connections(mut jobs: Receiver<Job>) {
    while let Some(job) = jobs.recv().await {
        let outcome = panic::catch_unwind(AssertUnwindSafe(|| process(job)));
        if outcome.is_err() {
            tracing::error!("request handler panicked; scheduling restart");
            request_graceful_restart();
        }
    }
}
```

## Don't
```rust
use std::panic;

fn run_forever(jobs: impl Iterator<Item = Job>) {
    for job in jobs {
        // Swallows the panic and keeps going on potentially corrupted
        // shared state, forever.
        let _ = panic::catch_unwind(|| process(job));
    }
}
```

## Notes
Library code in particular should not catch a panic and resume as if nothing happened — the caller, not the library, owns the restart decision. `catch_unwind` also requires the closure to be `UnwindSafe` (or wrapped in `AssertUnwindSafe`), which is itself a signal to double-check what state the closure touches before assuming it is fine to keep using.
