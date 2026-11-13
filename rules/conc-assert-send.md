---
id: conc-assert-send
title: Statically assert that public types and futures stay Send
category: concurrency
severity: info
tier: 1
enforceability: judge
tags: [send, futures, static-assert, ecosystem]
applies_to:
  - "public type intended for use across threads or with tokio::spawn"
  - "explicit Future impl on a public type"
  - "async fn whose future is spawned on a multi-threaded runtime"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-TYPES-SEND
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Public types, and the futures your public async entry points produce, should generally be `Send`. Pin that expectation down with a compile-time assertion so a later change that accidentally makes the type `!Send` fails the build instead of surfacing as a confusing error at a caller's `tokio::spawn`.

## Why
Work-stealing runtimes like Tokio require a spawned future to be `Send`. A public type that is `!Send` — often because it gained an `Rc` or `RefCell` field, or because an `async fn` started holding one across an `.await` — silently breaks every downstream user who spawns it, and the resulting compiler error points at the caller's `spawn` call, far from the actual cause. A static assertion at the definition site catches the regression exactly where it was introduced.

## Do
```rust
struct Client {
    inner: std::sync::Arc<InnerClient>,
}
struct InnerClient;

const fn assert_send<T: Send>() {}
const _: () = assert_send::<Client>();

async fn fetch(client: &Client) -> Result<String, std::io::Error> {
    Ok(String::new())
}

fn assert_future_send<T: Send>(_: T) {}
_ = assert_future_send(fetch(&Client { inner: std::sync::Arc::new(InnerClient) }));
```

## Don't
```rust
use std::rc::Rc;

// Silently !Send; a caller only finds out when tokio::spawn refuses
// to compile, with no assertion pointing back here.
struct Client {
    inner: Rc<InnerClient>,
}
struct InnerClient;
```

## Notes
A default use that is genuinely instantaneous and never held across an `.await` may legitimately be `!Send` — do not force `Send` onto types that have no reason to cross a thread. You do not need to assert every method's future individually; validate the type itself and your main async entry points.
