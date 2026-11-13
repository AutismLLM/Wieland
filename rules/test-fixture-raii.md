---
id: test-fixture-raii
title: Clean up test resources with RAII guards, not manual teardown
category: testing
severity: warn
tier: 1
enforceability: judge
tags: [cleanup, raii, fixtures, drop]
applies_to:
  - "tests that create temp files, env vars, or servers"
  - "manual remove_file / remove_var after the assertions"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-fixture-raii
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Tie test setup to a value whose `Drop` performs teardown (a `NamedTempFile`, `TempDir`, or a custom guard) so cleanup runs even when the test panics.

## Why
Teardown written as a statement after the assertions is skipped whenever an earlier assertion panics, leaking temp files, leaving env vars set, or keeping servers alive — which then pollutes and flakes other tests. `Drop` runs during unwinding, so an RAII guard cleans up unconditionally.

## Do
```rust
use tempfile::NamedTempFile;

#[test]
fn processes_file_contents() {
    let file = NamedTempFile::new().unwrap();
    std::fs::write(file.path(), "test data").unwrap();

    let result = process_file(file.path());

    assert!(result.is_ok());
}
```

## Don't
```rust
#[test]
fn processes_file_contents() {
    let path = "/tmp/test_file.txt";
    std::fs::write(path, "test data").unwrap();

    let result = process_file(path);

    std::fs::remove_file(path).unwrap();
    assert!(result.is_ok());
}
```

## Notes
For resources without a ready-made guard (env vars, database transactions), write a small struct that captures the original state and restores it in `Drop`. Since the 2024 edition `env::set_var` is `unsafe`; keep env-touching tests single-threaded.
