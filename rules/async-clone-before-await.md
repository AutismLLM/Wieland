---
id: async-clone-before-await
title: Clone the owned data you need before an await instead of borrowing across it
category: async
severity: warn
tier: 0
enforceability: judge
tags: [send, lifetimes, borrows]
applies_to:
  - "reference held across an .await point"
  - "future required to be Send for tokio::spawn"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-clone-before-await
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Before an `.await`, clone (or `Arc`-clone) the data you still need afterward, so the future captures owned data instead of a borrow held across suspension.

## Why
A borrow held across `.await` is stored inside the future and extends its lifetime, which triggers borrow-checker errors or makes the future `!Send` and unspawnable on a multi-threaded runtime. Cloning the needed value — or cloning the cheap `Arc` handle — leaves the future holding only owned data. Clone the smallest field you actually use so the cost stays minimal.

## Do
```rust
use std::sync::Arc;

async fn process(data: Arc<Data>) {
    let items = data.items.clone();
    expensive_async_operation().await;
    use_items(&items);
}
```

## Don't
```rust
use std::sync::Arc;

async fn process(data: Arc<Data>) {
    let slice = &data.items[..];
    expensive_async_operation().await;
    use_slice(slice);
}
```

## Notes
When you need the whole value afterward, keep the cheap `Arc` clone rather than deep-cloning the payload. Confine a borrow you only need before the await to its own block so it is dropped before suspension.
