---
id: async-try-join
title: Use try_join! for concurrent fallible operations with fail-fast on error
category: async
severity: warn
tier: 1
enforceability: judge
tags: [concurrency, error-handling, join]
applies_to:
  - "several fallible futures awaited in sequence with ?"
  - "concurrent operations that should abort on first error"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-try-join
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Run concurrent fallible futures with `try_join!` (or `futures::future::try_join_all` for a collection) so the first `Err` returns immediately.

## Why
Awaiting fallible futures in sequence with `?` costs the sum of their durations and gains no early exit — a late failure means you already paid for the earlier successes. `try_join!` runs them concurrently and short-circuits the moment any one fails, giving both parallelism and fail-fast behavior in a single expression.

## Do
```rust
use tokio::try_join;

async fn fetch_all() -> anyhow::Result<(A, B, C)> {
    let (a, b, c) = try_join!(
        fetch_a(),
        fetch_b(),
        fetch_c(),
    )?;
    Ok((a, b, c))
}
```

## Don't
```rust
async fn fetch_all() -> anyhow::Result<(A, B, C)> {
    let a = fetch_a().await?;
    let b = fetch_b().await?;
    let c = fetch_c().await?;
    Ok((a, b, c))
}
```

## Notes
On the first error the other branches are dropped at their next `.await`, so trailing cleanup in a sibling future may not run — use a `Drop` guard for cleanup that must happen. When you want every result regardless of failures, use `join_all` and filter the `Result`s.
