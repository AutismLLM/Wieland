---
id: proj-build-rs-minimal
title: Keep build.rs minimal, deterministic, and scoped with rerun directives
category: project-layout
severity: warn
tier: 1
enforceability: judge
tags: [cargo, build-script, reproducibility]
applies_to:
  - "build.rs"
  - "build scripts that make network calls or parse rustc --version"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-build-rs-minimal
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep `build.rs` short, deterministic, and idempotent: emit narrow `rerun-if-changed`/`rerun-if-env-changed` directives, never touch the network, and probe compiler capabilities instead of parsing version strings.

## Why
Cargo re-runs a build script on every build unless it emits rerun directives, and overly broad directives force needless recompiles. Non-determinism (timestamps, UUIDs, network fetches) breaks reproducible builds, caching, and offline workflows. Parsing `rustc --version` is fragile; `autocfg` probes the real toolchain by compiling snippets.

## Do
```rust
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/generated.rs");
    println!("cargo::rerun-if-env-changed=MY_BUILD_FLAG");

    let ac = autocfg::new();
    ac.emit_has_type("std::collections::BTreeMap");

    if std::env::var("MY_BUILD_FLAG").is_ok() {
        println!("cargo::rustc-cfg=my_feature");
    }
}
```

## Don't
```rust
use std::process::Command;

fn main() {
    let output = Command::new("rustc").arg("--version").output().unwrap();
    let version = String::from_utf8(output.stdout).unwrap();
    if version.contains("1.8") {
        println!("cargo::rustc-cfg=has_feature");
    }
    let _resp = reqwest::blocking::get("https://example.com/schema.json").unwrap();
}
```

## Notes
List every file the script reads so stale output cannot slip through, write only inside `OUT_DIR`, and extract complex codegen into a separate library crate rather than growing `build.rs`.
