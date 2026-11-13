---
id: api-impl-io
title: Accept impl Read/Write instead of a concrete I/O source (sans-IO)
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [io, sans-io, composability, traits]
applies_to:
  - "fn parameter typed as File or TcpStream for one-shot parsing"
  - "parsing/business logic tangled with a concrete I/O source"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-IMPL-IO
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a function only needs to perform one-shot I/O — reading input to parse it, writing output once — accept `impl std::io::Read` or `impl std::io::Write` instead of a concrete type like `File` or `TcpStream`, so the I/O source is a caller-supplied detail instead of a hard-coded dependency.

## Why
A function that takes `File` forces every caller through a filesystem, even one whose data actually comes from the network, `stdin`, or an in-memory buffer — the network caller must write to a temp file first just to satisfy the signature. Accepting `impl Read` outsources the I/O to the caller and lets the same function parse files, sockets, `Stdin`, `&[u8]`, and any other reader with zero changes, giving N-source-by-M-consumer composability instead of one hard-wired path.

## Do
```rust
use std::io::Read;

fn parse_data(mut data: impl Read) -> std::io::Result<Vec<String>> {
    let mut buf = String::new();
    data.read_to_string(&mut buf)?;
    Ok(buf.lines().map(str::to_owned).collect())
}

let _ = parse_data(&b"a\nb\nc"[..]);
let _ = parse_data(std::io::stdin());
```

## Don't
```rust
use std::fs::File;
use std::io::Read;

// Any caller whose data is not already on disk must write a temp
// file first, just to satisfy this signature.
fn parse_data(mut file: File) -> std::io::Result<Vec<String>> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf.lines().map(str::to_owned).collect())
}
```

## Notes
Synchronous code should bound on `std::io::Read`/`std::io::Write`; async functions targeting more than one runtime should bound on `futures::io::AsyncRead`/`AsyncWrite` instead of a runtime-specific type. Types that need continuous, runtime-specific I/O (not a one-shot parse) are a different concern and should be abstracted behind your own trait rather than forced into `impl Read`.
