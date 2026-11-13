---
id: async-fn-over-future
title: Declare async fn directly instead of hand-written impl Future returns
category: async
severity: info
tier: 1
enforceability: judge
tags: [async, api-design, ergonomics]
applies_to:
  - "fn returning impl Future<Output = T>"
  - "fn body wrapping its result in an async block"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-ASYNC-FN
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Declare functions `async fn foo() -> T` rather than `fn foo() -> impl Future<Output = T>` with a manual `async` block, unless a trait definition or an explicit bound the sugar cannot express genuinely requires the desugared form.

## Why
`async fn` is the idiomatic sugar the desugared form exists to produce: the signature reads normally and the body needs no extra `async {}` wrapper. Spelling out `-> impl Future` by hand duplicates that machinery, adds a nesting level to the body, and buys nothing unless you specifically need to name extra bounds the sugar omits, such as `+ Send` on the returned future.

## Do
```rust
async fn fetch(id: u64) -> Result<String, std::io::Error> {
    Ok(format!("row-{id}"))
}
```

## Don't
```rust
use std::future::Future;

fn fetch(id: u64) -> impl Future<Output = Result<String, std::io::Error>> {
    async move { Ok(format!("row-{id}")) }
}
```

## Notes
Keep the desugared `-> impl Future` form when you must add bounds `async fn` cannot express directly, such as `+ Send + 'static` on the returned future for a multi-threaded runtime, or inside a trait definition that needs to stay `dyn`-compatible — see native `async fn` in traits for the common case instead.
