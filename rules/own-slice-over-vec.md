---
id: own-slice-over-vec
title: Accept &[T] not &Vec<T>, and &str not &String
category: ownership
severity: info
tier: 0
enforceability: check
tags: [api, ergonomics, flexibility]
applies_to:
  - "fn parameter &Vec<T>"
  - "fn parameter &String"
  - "fn parameter &PathBuf"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['&Vec<', '&mut Vec<']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-slice-over-vec
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Take the borrowed view type in function parameters: `&[T]` instead of `&Vec<T>`, `&str` instead of `&String`, `&Path` instead of `&PathBuf`.

## Why
`&[T]` accepts a slice from a `Vec`, an array, a boxed slice, or any other source; `&Vec<T>` only accepts a `Vec`. The same holds for `&str` versus `&String`. Because deref coercion converts `&Vec<T> -> &[T]` and `&String -> &str` automatically, the slice signature is strictly more flexible for callers at no cost to the implementation, which never needed the extra `Vec`/`String` capabilities anyway.

## Do
```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn greet(name: &str) {
    println!("Hello, {name}");
}
```

## Don't
```rust
fn sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

fn greet(name: &String) {
    println!("Hello, {name}");
}
```

## Notes
Accept an owned type only when the function stores it — a constructor taking `prefix: String` (or `impl Into<String>` for flexibility) is fine because it keeps the value. For paths, `impl AsRef<Path>` accepts `&Path`, `&PathBuf`, `&str`, and `&String` at once.
