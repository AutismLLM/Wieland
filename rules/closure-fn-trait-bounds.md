---
id: closure-fn-trait-bounds
title: Bound a callback with the weakest Fn trait its body actually needs
category: closures
severity: info
tier: 1
enforceability: judge
tags: [traits, fn-once, fn-mut, api-design]
applies_to:
  - "generic function taking a closure parameter"
  - "F: Fn bound on a callback"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#closure-fn-trait-bounds
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Bound a callback parameter with the weakest of `FnOnce`, `FnMut`, `Fn` that the function body requires, so the widest set of callers is accepted.

## Why
`Fn: FnMut: FnOnce`: every closure implements `FnOnce`, mutating closures implement `FnMut`, and only read-only closures implement `Fn`. Requiring `Fn` when you call the closure once needlessly rejects move-consuming closures; requiring `FnMut` when you never mutate rejects nothing but signals the wrong contract. Pick the trait by how you actually invoke the closure: once, repeatedly-with-mutation, or repeatedly-read-only.

## Do
```rust
fn run_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn retry<F: FnMut() -> bool>(mut f: F, attempts: usize) -> bool {
    for _ in 0..attempts {
        if f() {
            return true;
        }
    }
    false
}

fn demo() {
    let s = String::from("hello");
    let result = run_once(move || s.to_uppercase());
    assert_eq!(result, "HELLO");

    let mut count = 0usize;
    let found = retry(|| { count += 1; count == 3 }, 5);
    assert!(found);
}
```

## Don't
```rust
fn run_once_bad<F: Fn() -> String>(f: F) -> String {
    f()
}

fn demo_bad() {
    let s = String::from("hello");
    let _ = run_once_bad(|| String::from("ok"));
}
```

## Notes
`FnMut` bounds require `mut f` at the call site. Standard-library shapes to mirror: `Iterator::map` takes `FnMut`; `thread::spawn` takes `FnOnce + Send + 'static`.
