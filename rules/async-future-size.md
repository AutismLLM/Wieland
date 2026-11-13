---
id: async-future-size
title: Keep hot async fn future sizes small; measure with size_of_val
category: async
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, hot-path]
applies_to:
  - "async fn spawned or polled thousands of times per second"
  - "large parameters or locals held across .await points"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-ASYNC-STACK-SIZE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For an `async fn` on a hot path (spawned or polled thousands of times per second), measure the resulting `Future`'s size with `size_of_val` and shrink it by returning `impl Future` from a plain function, moving non-async setup outside the `async` block, and keeping large values out of scope across `.await` points.

## Why
Everything that lives across an `.await` point — including parameters captured by the async block — becomes a field of the compiler-generated state machine type for that future, whether or not it is used after the await. A large parameter or a stray local that happens to survive a `.await` inflates the future's size, and that size is paid again as a stack-to-heap memcpy whenever the future is boxed or handed to `spawn`. Wrapping setup logic in a plain `fn` that returns `impl Future` keeps arguments out of the future type entirely.

## Do
```rust
use std::future::Future;

fn hot(args: Args) -> impl Future<Output = Result<Response, Error>> {
    let request = args.into_request();
    async move { send(request).await }
}

#[test]
fn future_size_is_bounded() {
    let f = hot(Args::default());
    assert!(std::mem::size_of_val(&f) <= 256);
}
```

## Don't
```rust
async fn hot(args: Args) -> Result<Response, Error> {
    let large_local = [0_u8; 1024];
    let request = args.into_request();
    let response = send(request).await;
    dbg!(&large_local);
    response
}
```

## Notes
Only bother for functions instantiated or awaited at high frequency; the vast majority of async functions have negligible future size and this check is wasted effort on them. `Either`-style combinators (e.g. `futures::future::Either`) let branches with different future types unify into one `impl Future` return without boxing.
