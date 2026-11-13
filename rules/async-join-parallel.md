---
id: async-join-parallel
title: Run independent futures concurrently with join! instead of awaiting in sequence
category: async
severity: warn
tier: 1
enforceability: judge
tags: [concurrency, latency, join]
applies_to:
  - "several independent .await calls in sequence"
  - "multiple independent I/O or network calls"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-join-parallel
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When several futures are independent, drive them together with `join!` (or `try_join!` for fallible ones) rather than awaiting each in sequence.

## Why
Awaiting futures one after another costs the sum of their durations even though none depends on another's result. `join!` polls them concurrently on the same task, so the total latency drops to roughly the slowest one. For a handful of independent API calls or file reads this is a large, free latency win.

## Do
```rust
use tokio::join;

async fn fetch_data() -> (User, Posts, Comments) {
    let (user, posts, comments) = join!(
        fetch_user(),
        fetch_posts(),
        fetch_comments(),
    );
    (user, posts, comments)
}
```

## Don't
```rust
async fn fetch_data() -> (User, Posts, Comments) {
    let user = fetch_user().await;
    let posts = fetch_posts().await;
    let comments = fetch_comments().await;
    (user, posts, comments)
}
```

## Notes
Do not use `join!` when a later future depends on an earlier result, when you want first-to-finish semantics (use `select!`), or for a dynamic count of futures (use `join_all` / `JoinSet`). Cap fan-out with `buffer_unordered` or a `Semaphore` when the future count is large.
