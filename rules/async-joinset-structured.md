---
id: async-joinset-structured
title: Manage a dynamic set of spawned tasks with JoinSet
category: async
severity: info
tier: 1
enforceability: judge
tags: [tasks, structured-concurrency, joinset]
applies_to:
  - "Vec<JoinHandle> collected for join_all"
  - "variable number of spawned tasks"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-joinset-structured
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For a variable or dynamic collection of spawned tasks, use `tokio::task::JoinSet` instead of hand-collecting `JoinHandle`s into a `Vec`.

## Why
A `Vec<JoinHandle>` plus `join_all` yields results only once every task finishes and gives no way to add tasks, react as each completes, or cancel the group. `JoinSet` lets you spawn dynamically, pull results with `join_next` as they arrive, and aborts all outstanding tasks when it is dropped — structured concurrency where the task set's lifetime is tied to a scope.

## Do
```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();
for url in urls {
    set.spawn(fetch(url.clone()));
}

while let Some(result) = set.join_next().await {
    match result {
        Ok(Ok(data)) => process(data),
        Ok(Err(e)) => log::error!("task failed: {e}"),
        Err(e) => log::error!("task panicked: {e}"),
    }
}
```

## Don't
```rust
let mut handles: Vec<JoinHandle<Result<Data>>> = Vec::new();
for url in urls {
    handles.push(tokio::spawn(fetch(url)));
}

let results = futures::future::join_all(handles).await;
```

## Notes
`JoinSet` aborts remaining tasks on drop, so an early return cancels them cleanly; call `abort_all` then drain with `join_next` when you want to wait for aborted tasks to finish.
