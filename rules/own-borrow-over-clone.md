---
id: own-borrow-over-clone
title: Prefer borrowing over cloning to read data
category: ownership
severity: warn
tier: 0
enforceability: judge
tags: [performance, allocation, hot-path, ergonomics]
applies_to:
  - ".clone()"
  - "fn taking &String or &Vec<T>"
  - "clone inside a loop"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-borrow-over-clone
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Borrow with `&T` to read data instead of calling `.clone()`, and clone only when you genuinely need an owned value.

## Why
Cloning allocates fresh memory and copies the contents; borrowing is free. An unnecessary clone in a hot path or over a large structure is pure waste, and a clone inside a loop multiplies that cost by the iteration count. Reserve clones for the cases where ownership is actually required — storing the value, sending it to another thread, or satisfying a `'static` bound.

## Do
```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn process_all(items: &[String]) {
    for item in items {
        handle(item);
    }
}
```

## Don't
```rust
fn count_words(text: &String) -> usize {
    let owned = text.clone();
    owned.split_whitespace().count()
}

fn process_all(items: &[String]) {
    for item in items {
        let copy = item.clone();
        handle(&copy);
    }
}
```

## Notes
Cloning is correct when you must own the data: inserting into a collection, moving into a `std::thread::spawn` closure for a `'static` bound, or working with `Copy` types where `let y = x;` is a cheap bit-copy, not a heap clone. `Cow<T>` lets you borrow in the common case and clone only when a mutation forces it.
