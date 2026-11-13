---
id: async-select-racing
title: Race futures with select! to act on whichever completes first
category: async
severity: info
tier: 1
enforceability: judge
tags: [select, racing, timeout]
applies_to:
  - "operation that needs a timeout"
  - "work that must yield to cancellation or a competing future"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-select-racing
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `tokio::select!` to race futures and handle whichever finishes first — operation vs timeout, work vs cancellation, or competing alternatives.

## Why
`select!` polls multiple futures concurrently, runs the branch that completes first, and drops the rest. This expresses "whichever finishes first" directly, which sequential `match … await` cannot, and it makes timeouts and cancellation trivial instead of hand-rolled polling loops. Add `biased;` when you need deterministic branch priority instead of the default random choice among ready branches.

## Do
```rust
use tokio::select;
use tokio::time::{sleep, Duration};

async fn fetch_with_timeout() -> Result<Data, Error> {
    select! {
        result = fetch_data() => result,
        _ = sleep(Duration::from_secs(5)) => Err(Error::Timeout),
    }
}
```

## Don't
```rust
use std::time::{Duration, Instant};

async fn fetch_with_timeout() -> Option<Data> {
    let start = Instant::now();
    loop {
        if start.elapsed() > Duration::from_secs(5) {
            return None;
        }
    }
}
```

## Notes
Dropped branches are cancelled at their next `.await`, so every branch future must be cancellation-safe or hold its partial state outside the branch. For a dynamic list of futures use `futures::future::select_all` since `select!` needs static branches.
