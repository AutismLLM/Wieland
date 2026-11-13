---
id: type-path-not-string
title: Use Path/PathBuf, not String, for anything that is a filesystem path
category: types
severity: warn
tier: 1
enforceability: judge
tags: [types, path, api-design]
applies_to:
  - "struct field or function parameter holding a filesystem path as String"
  - "return type of a function that produces a filesystem path"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-STRONG-TYPES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use the strongest applicable `std` type for the domain of a value. Anything that represents an OS filesystem path is `Path`/`PathBuf` (or `OsStr`/`OsString` for non-path OS strings), never `String`/`&str`.

## Why
`String` guarantees valid UTF-8, but real filesystem paths are not guaranteed to be UTF-8 on every platform Rust supports. Using `String` for a path either silently mishandles non-UTF-8 paths or forces lossy round-tripping through `to_string_lossy`. `Path`/`PathBuf` carry the platform's actual path semantics (separators, prefixes, `OsStr` comparisons) and every path-taking `std` API already expects them, so using the right type end to end removes conversions instead of adding them.

## Do
```rust
use std::path::{Path, PathBuf};

pub struct Config {
    pub log_dir: PathBuf,
}

pub fn resolve_log_file(config: &Config, name: &str) -> PathBuf {
    config.log_dir.join(name)
}

fn read_config(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

## Don't
```rust
pub struct Config {
    pub log_dir: String,
}

pub fn resolve_log_file(config: &Config, name: &str) -> String {
    format!("{}/{}", config.log_dir, name)
}

fn read_config(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

## Notes
This cuts both ways: a purely numeric value at a public API boundary (`window_size() -> u32`) should stay a plain number, not a `Saturating<u32>` or `NonZero<u32>`, even if the type is used internally — callers expect ordinary arithmetic types at the surface. Reach for `impl AsRef<Path>` (see `api-impl-asref`) when a function only needs to borrow the path.
