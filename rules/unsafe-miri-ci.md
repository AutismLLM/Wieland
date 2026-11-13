---
id: unsafe-miri-ci
title: Run cargo miri test in CI for every crate that contains unsafe code
category: unsafe
severity: info
tier: 1
enforceability: judge
tags: [ci, testing, undefined-behavior]
applies_to:
  - "crates containing unsafe blocks"
  - "CI configuration"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-miri-ci
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Run `cargo miri test` in CI for every crate that contains `unsafe` code.

## Why
Miri is the only tool that dynamically detects undefined behavior at test time: out-of-bounds accesses, use-after-free, reads of uninitialized memory, invalid pointer provenance, data races, and violations of the Stacked/Tree Borrows aliasing models. Static analysis and code review miss subtle UB that manifests only at specific layouts; Miri's interpreted execution catches it unconditionally. The standard library, tokio, and serde all run Miri in CI before merging unsafe changes.

## Do
```yaml
# .github/workflows/miri.yml
name: Miri
on: [push, pull_request]
jobs:
  miri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install nightly toolchain with Miri
        run: |
          rustup toolchain install nightly --component miri
          rustup override set nightly
          cargo miri setup
      - name: Run Miri
        env:
          MIRIFLAGS: "-Zmiri-strict-provenance"
        run: cargo miri test --all-features
```

## Don't
```yaml
# CI that tests but never runs Miri — unsafe code ships unverified.
- name: Test
  run: cargo test --all-features
```

## Notes
Miri needs a nightly toolchain and runs 100–1000x slower than normal, so target a reduced test subset or a separate CI job if the full suite is prohibitive. Crates with zero `unsafe` gain nothing from it; skip generated code and proc-macro output you do not control (audit the generator instead).
