---
id: async-cancellation-token
title: Use CancellationToken for cooperative graceful shutdown of tasks
category: async
severity: warn
tier: 1
enforceability: judge
tags: [shutdown, cancellation, tasks]
applies_to:
  - "long-running spawned task / worker loop"
  - "dropping a JoinHandle to stop a task"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-cancellation-token
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Signal graceful shutdown with a `tokio_util::sync::CancellationToken` selected against your work, rather than dropping a `JoinHandle` or flipping an `AtomicBool`.

## Why
Dropping a `JoinHandle` detaches the task but does not stop it — the loop keeps running in the background. An `AtomicBool` flag is only observed between awaits, so a task blocked inside an `.await` cannot react. A `CancellationToken` gives every task an awaitable cancellation signal it can race in `select!`, enabling prompt wake-up and clean resource cleanup.

## Do
```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();

let handle = tokio::spawn({
    let token = token.clone();
    async move {
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    cleanup().await;
                    break;
                }
                _ = do_work() => {}
            }
        }
    }
});

token.cancel();
handle.await.unwrap();
```

## Don't
```rust
let handle = tokio::spawn(async {
    loop {
        do_work().await;
    }
});

drop(handle);
```

## Notes
Use `child_token()` to build a shutdown hierarchy where cancelling a parent cancels all children, and `drop_guard()` to auto-cancel when a scope exits.
