---
id: proj-sys-crate-vendor-source
title: Vendor and verify native sources in -sys crates instead of shelling to a build system
category: project-layout
severity: warn
tier: 2
enforceability: check
tags: [ffi, sys-crate, build-script]
applies_to:
  - "foo-sys crate wrapping a native foo library"
  - "build.rs that compiles or links a vendored C/C++ library"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['Command::new\("(make|cmake|configure|\./configure)"\)']
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SYS-CRATES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A `foo-sys` crate wrapping a native `foo` library must govern its own build from `build.rs` using the [`cc`](https://crates.io/crates/cc) crate, not by invoking the upstream Makefile, `configure`, or CMake. Embed a verifiable copy of the upstream sources (with the origin Git URL and commit hash recorded) inside the crate, pre-generate any `bindgen` glue where possible, and support both static linking and dynamic loading via [`libloading`](https://crates.io/crates/libloading).

## Why
A `-sys` crate that delegates to an external build system pushes its native prerequisites onto every downstream consumer, reproducing the exact "just works" failure the pair of crates was meant to hide behind a Rust API. Vendoring and compiling with `cc` keeps the build fully governed by Cargo; recording the source hash keeps that vendored copy auditable.

## Do
```rust
fn main() {
    cc::Build::new()
        .file("vendor/foo/src/foo.c")
        .include("vendor/foo/include")
        .compile("foo");

    println!("cargo::rerun-if-changed=vendor/foo");
}
```
```text
vendor/foo/SOURCE.md
  git = "https://github.com/upstream/foo"
  commit = "a1b2c3d4e5f6..."
```

## Don't
```rust
use std::process::Command;

fn main() {
    // requires the user to have `make` and the native build toolchain installed
    Command::new("make")
        .current_dir("vendor/foo")
        .status()
        .expect("make must be installed to build foo-sys");
}
```

## Notes
When static linking is not desirable, fall back to `libloading` at runtime rather than hard-linking a system library that may not exist. Allow the vendored source root to be overridden via an environment variable so hermetic build environments can substitute their own copy.
