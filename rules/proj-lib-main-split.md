---
id: proj-lib-main-split
title: Keep main.rs a thin entry point and put logic in lib.rs
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [layout, testability, binaries]
applies_to:
  - "src/main.rs holding substantial application logic"
  - "a binary crate you want to integration-test"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-lib-main-split
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Put application logic in `lib.rs` and keep `main.rs` a thin shim that parses inputs and calls into the library.

## Why
Integration tests in `tests/` can only reach your library crate, not code that lives in `main.rs`, so logic stranded in the binary is untestable and unreusable. A `lib.rs`/`main.rs` split makes the same code testable, reusable across multiple binaries, and leaves the entry point trivial.

## Do
```rust
// src/main.rs — thin entry point
use my_app::{run, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    run(config)
}

// src/lib.rs — all the logic, reachable from tests
pub mod config;
pub mod database;
pub use config::Config;

pub fn run(config: Config) -> anyhow::Result<()> {
    let db = database::connect(&config.db_url)?;
    db.serve()
}
```

## Don't
```rust
// src/main.rs — everything inline, untestable from integration tests
fn main() {
    let args = parse_args();
    let config = load_config(&args.config_path).unwrap();
    let db = connect_database(&config.db_url).unwrap();
    // hundreds of lines of logic no test in tests/ can reach...
}
```

## Notes
This also sets up cleanly for multiple binaries: shared code stays in `lib.rs` while each `src/bin/*.rs` is a small `main`.
