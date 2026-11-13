---
id: async-async-fn-bounds
title: Bound higher-order async functions with AsyncFn instead of F Fn plus Fut Future
category: async
severity: info
tier: 1
enforceability: judge
tags: [traits, closures, ergonomics]
applies_to:
  - "generic function taking an async callback"
  - "F: Fn() -> Fut, Fut: Future bound"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-async-fn-bounds
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Express a higher-order async function bound with `AsyncFn`/`AsyncFnMut`/`AsyncFnOnce` rather than the two-generic `F: Fn() -> Fut, Fut: Future` pattern.

## Why
The `AsyncFn` family stabilized in Rust 1.85 and states a single, readable constraint that also links the returned future's lifetime to the call. The old `F: Fn() -> Fut, Fut: Future` pattern cannot accept an `async ||` closure that borrows from its environment across the call, because the future's lifetime is unconnected to the closure. `AsyncFn` fixes this structurally, mirroring `Fn`/`FnMut`/`FnOnce` semantics.

## Do
```rust
async fn retry<F, T, E>(times: usize, f: F) -> Result<T, E>
where
    F: AsyncFn() -> Result<T, E>,
{
    let mut i = 0;
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                i += 1;
                if i >= times {
                    return Err(e);
                }
            }
        }
    }
}
```

## Don't
```rust
use std::future::Future;

async fn retry<F, Fut, T, E>(times: usize, f: F) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut i = 0;
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                i += 1;
                if i >= times {
                    return Err(e);
                }
            }
        }
    }
}
```

## Notes
On a multi-threaded Tokio runtime a spawned future must be `Send`; add `+ Send` to the bound or use a `Send` wrapper when the closure body requires it.
