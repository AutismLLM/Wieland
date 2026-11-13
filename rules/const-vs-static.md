---
id: const-vs-static
title: Use const for an inlined value and static for a single addressed instance
category: const-eval
severity: warn
tier: 2
enforceability: check
tags: [globals, memory, mutability]
applies_to:
  - "static mut global state"
  - "large lookup table declared as const"
  - "tiny value declared as static"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['static\s+mut\s']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#const-vs-static
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `const` for small values inlined at each use site and `static` for large tables or when a `&'static T` is required; never use `static mut`.

## Why
A `const` item is substituted inline at every use — no fixed address, no storage — which suits small config values, bitmasks, and magic numbers. A `static` lives at one address for the whole program and exposes a `'static` reference, so it fits large lookup tables you do not want duplicated. Referencing `static mut` is a hard error in the 2024 edition; for mutable global state use atomics or `OnceLock`/`LazyLock` instead.

## Do
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, OnceLock};

const MAX_RETRIES: u32 = 3;
const FLAG_MASK: u8 = 0b0000_1111;

static LOOKUP: [u8; 256] = [0u8; 256];
static APP_NAME: &str = "my-app";

static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

fn record_request() {
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
}

static CONFIG_PATH: LazyLock<String> = LazyLock::new(|| {
    std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/etc/app/config.toml".to_owned())
});

static GREETING: OnceLock<String> = OnceLock::new();
```

## Don't
```rust
const LOOKUP: [u8; 256] = [0u8; 256];

static mut COUNTER: u64 = 0;

static TIMEOUT_MS: u64 = 5000;
```

## Notes
Use `const` for the common cases: numbers, bools, tiny arrays, string literals. Reach for `static` for large tables, `&'static T` needs, atomics for mutable counters, and `LazyLock`/`OnceLock` for lazily or single-writer initialized globals.
