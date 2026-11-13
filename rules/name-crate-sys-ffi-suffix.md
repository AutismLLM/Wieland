---
id: name-crate-sys-ffi-suffix
title: Suffix FFI crates -sys for imports and -ffi for exports
category: naming
severity: info
tier: 1
enforceability: judge
tags: [ffi, crates, cargo]
applies_to:
  - "crate name in Cargo.toml that only binds an existing C library"
  - "crate name in Cargo.toml that exports a C ABI for other languages to call"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-FFI-NAMING
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name a crate that declares bindings into an existing C-style library `<name>-sys`. Name a crate that exposes a C-style API for other languages to call into `<name>-ffi`.

## Why
Neither suffix is guessable from the crate's Rust API alone — both are typically thin, mostly-unsafe, low-level surfaces meant to be wrapped by something more ergonomic. A consistent suffix tells every downstream developer, at a glance in `Cargo.toml` or crates.io, which direction the FFI goes: `-sys` pulls a foreign library in, `-ffi` pushes a Rust library out. Established crates like `libz-sys`, `openssl-sys`, and `wasmtime-c-api` (an `-ffi`-style crate) follow this so tooling and humans can classify a crate without opening its source.

## Do
```toml
# zstd-sys/Cargo.toml — imports the C zstd library
[package]
name = "zstd-sys"
version = "2.0.0"
edition = "2024"

[lib]
crate-type = ["lib"]
```

```toml
# widget-core-ffi/Cargo.toml — exports widget-core's API as a C ABI
[package]
name = "widget-core-ffi"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "staticlib"]
```

## Don't
```toml
# ambiguous: does this call into zstd, or expose Rust to callers?
[package]
name = "zstd-bindings"
version = "2.0.0"
edition = "2024"
```

## Notes
Minor variants are accepted ecosystem-wide when a plain `-sys` name was abandoned or squatted (`foo-sys2`), or when `_` replaces `-` for a module-name match. Prefer `-sys` over `-bindings`, `-raw`, or `-native`, and prefer `-ffi` over `-capi` or `-cffi` unless the ecosystem you are targeting has its own established convention.
