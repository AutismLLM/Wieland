---
id: name-consts-screaming
title: Name constants and statics in SCREAMING_SNAKE_CASE
category: naming
severity: warn
tier: 2
enforceability: check
tags: [naming, constants, casing]
applies_to:
  - "const and static item declarations"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['const [A-Za-z0-9_]*[a-z][A-Za-z0-9_]*:', 'static [A-Za-z0-9_]*[a-z][A-Za-z0-9_]*:']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-consts-screaming
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give every `const` and `static` a `SCREAMING_SNAKE_CASE` name.

## Why
Constants and statics are known at compile time and live for the whole program; screaming case makes them visually distinct from runtime bindings at every use site. The convention is universal and the compiler warns on violations, so any other casing reads as a mistake.

## Do
```rust
use std::sync::atomic::AtomicU64;

const MAX_CONNECTIONS: u32 = 100;
const BUFFER_SIZE: usize = 4096;

static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);
```

## Don't
```rust
use std::sync::atomic::AtomicU64;

const maxConnections: u32 = 100;
const default_timeout: u64 = 30;

static globalCounter: AtomicU64 = AtomicU64::new(0);
```

## Notes
Associated constants in traits and `impl` blocks follow the same rule (`const MAX: usize = 256;`).
