---
id: mem-zero-copy
title: Borrow slices and &str instead of copying data into owned Strings and Vecs
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, borrowing]
applies_to:
  - "mapping parsed fields to String with to_string / to_vec"
  - "returning owned copies where a borrow would do"
  - "Cow for borrow-or-own"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-zero-copy
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When data can borrow from its source, return `&str`/`&[T]` (or `Cow` when ownership is conditional) instead of copying it into owned `String`/`Vec` values.

## Why
Copying allocates new memory and duplicates bytes; borrowing hands back a pointer and length into data that already exists. For parsing and slicing large buffers, working with references eliminates both the allocations and the copies, which dominate the cost for big inputs. A `Cow` lets you stay zero-copy in the common case and only allocate when the data must actually be modified.

## Do
```rust
struct Parsed<'a> {
    name: &'a str,
    value: &'a str,
}

fn parse(input: &str) -> Parsed<'_> {
    let (name, value) = input.split_once('=').unwrap();
    Parsed { name, value }
}
```

## Don't
```rust
struct Parsed {
    name: String,
    value: String,
}

fn parse(input: &str) -> Parsed {
    let (name, value) = input.split_once('=').unwrap();
    Parsed {
        name: name.to_string(),
        value: value.to_string(),
    }
}
```

## Notes
Copying is unavoidable when you must modify the data, when it has to outlive its source, or when it moves to another thread without shared ownership. In those cases `to_string()`/`to_vec()` is correct, and `Cow` can defer the copy to only when it is needed.
