---
id: async-tokio-fs
title: Use tokio::fs instead of std::fs inside async code
category: async
severity: warn
tier: 2
enforceability: check
tags: [filesystem, blocking, io]
applies_to:
  - "std::fs::read / read_to_string / write in an async fn"
  - "filesystem access on an executor thread"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['std::fs::', 'fs::read_to_string\(', 'fs::write\(']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-tokio-fs
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In async code, reach for `tokio::fs` (awaited) rather than the blocking `std::fs` calls.

## Why
`std::fs` operations block the calling thread until the syscall returns, and on an executor thread that halts every other task waiting to run there. `tokio::fs` wraps each operation in `spawn_blocking` and awaits it, so the executor stays responsive. It also composes with `try_join_all` for concurrent reads.

## Do
```rust
use tokio::fs;
use std::path::PathBuf;

async fn process_files(paths: &[PathBuf]) -> anyhow::Result<Vec<String>> {
    let mut contents = Vec::new();
    for path in paths {
        contents.push(fs::read_to_string(path).await?);
    }
    Ok(contents)
}
```

## Don't
```rust
use std::path::PathBuf;

async fn process_files(paths: &[PathBuf]) -> anyhow::Result<Vec<String>> {
    let mut contents = Vec::new();
    for path in paths {
        contents.push(std::fs::read_to_string(path)?);
    }
    Ok(contents)
}
```

## Notes
Reading config once at startup before the runtime spins up, or on a `current_thread` runtime, makes `std::fs` acceptable. Because `tokio::fs` uses `spawn_blocking` per call, batch many tiny reads with `try_join_all` rather than looping.
