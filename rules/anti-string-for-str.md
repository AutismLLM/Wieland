---
id: anti-string-for-str
title: Accept &str instead of &String in function signatures
category: anti-patterns
severity: warn
tier: 2
enforceability: gate
tags: [api-design, idiom, allocation]
applies_to:
  - "a function parameter typed &String"
rust_edition: "2024"
msrv: null
detect:
  clippy: [ptr_arg]
  grep: ['&String\b']
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#anti-string-for-str
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Take `&str` rather than `&String` for read-only string parameters.

## Why
`&String` is strictly less flexible than `&str`. A `&str` accepts owned `String`s (via deref coercion), string literals, and slices; `&String` demands exactly a `String`, forcing callers to allocate when they need not. Clippy's `ptr_arg` flags this directly.

## Do
```rust
fn greet(name: &str) {
    println!("Hello, {name}");
}

greet("Alice");
greet(&String::from("Bob"));
```

## Don't
```rust
fn greet(name: &String) {
    println!("Hello, {name}");
}

greet(&"Alice".to_string());
```

## Notes
Accept `&String` only when you need `String`-specific methods such as `capacity()`. For maximum flexibility over anything string-like, take `impl AsRef<str>` and call `.as_ref()` inside. The same rule applies to `&PathBuf` → `&Path` and `&OsString` → `&OsStr`.
