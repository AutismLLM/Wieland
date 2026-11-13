---
id: api-impl-asref
title: Accept impl AsRef<T> when a function only needs to borrow the inner data
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [conversion, asref, ergonomics]
applies_to:
  - "fn parameter of type &str or &Path"
  - "read-only access to borrowed data"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-impl-asref
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a function only reads borrowed data, take `impl AsRef<T>` so callers can pass any type that can be viewed as `T`, instead of pinning them to one concrete reference type.

## Why
`AsRef<T>` gives a cheap borrowed view with no ownership transfer and no allocation. A parameter of `impl AsRef<Path>` accepts `&str`, `String`, `&Path`, `PathBuf`, and `OsStr` alike, removing verbose `.as_ref()`/conversion noise at every call site. Reserve `impl Into<T>` for when the function must own the value; use `AsRef` when it only needs to read.

## Do
```rust
use std::path::Path;
use std::io;

fn read_file(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    std::fs::read(path.as_ref())
}

let _ = read_file("/etc/hostname");
let _ = read_file(std::path::PathBuf::from("/etc/hostname"));
```

## Don't
```rust
use std::path::Path;
use std::io;

fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    std::fs::read(path)
}

let _ = read_file(Path::new("/etc/hostname"));
```
