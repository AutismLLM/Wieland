---
id: macro-dont-lie-about-shape
title: Never let a macro change the async-ness, signature, or kind of the item it wraps
category: macros
severity: warn
tier: 1
enforceability: judge
tags: [macros, proc-macro, api-design, readability]
applies_to:
  - "an attribute macro applied to a fn, struct, or enum"
  - "a derive or attribute macro that rewrites the signature of its input"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-MACROS-DONT-LIE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A macro must not make the item it expands to lie about its own shape: it must not turn a `struct` into an `enum`, add or remove parameters from a function signature, or flip an item's `async`-ness, relative to what is written at the call site.

## Why
Readers and tools (rustfmt, rust-analyzer, an LLM reading the diff) reason from the syntax that is actually written. A macro that silently adds a hidden parameter or makes a plain `fn` callable with `.await` detaches what's written from what's happening — call sites become impossible to type-check by eye, and a caller cannot tell from the function definition alone how to invoke it.

## Do
```rust
// Attribute only adds instrumentation around the existing signature and
// async-ness; the wrapped function stays exactly as callable as written.
#[tracing::instrument]
async fn fetch_user(id: u64) -> Option<String> {
    Some(format!("user-{id}"))
}

fn main() {
    let _ = fetch_user(1);
}
```

## Don't
```rust
// Bad: expansion secretly injects a `token` parameter and makes the
// function async, so `foo()` at the call site no longer matches the
// signature a reader sees, and `.await` is required with no `async` in sight.
#[magic_transform]
fn foo() {}

async fn caller() {
    foo(some_token).await;
}
```

## Notes
Adding attributes, derives, or wrapping the body is fine — the boundary is the item's externally visible shape: its kind (struct/enum/fn), its parameter list, its return type, and its `async`/`unsafe`/`const` qualifiers must match what is written, not what the macro decided to make of it.
