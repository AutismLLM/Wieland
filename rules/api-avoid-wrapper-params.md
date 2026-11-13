---
id: api-avoid-wrapper-params
title: Keep smart pointers and interior-mutability wrappers out of public signatures
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api-design, smart-pointers, ergonomics]
applies_to:
  - "pub fn parameter typed Arc<T>, Rc<T>, Box<T>, or RefCell<T>"
  - "pub fn return type wrapping the result in a smart pointer"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-AVOID-WRAPPERS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep `Rc<T>`, `Arc<T>`, `Box<T>`, `RefCell<T>` and similar wrappers out of public function signatures; accept and return the plain type (`&T`, `&mut T`, `T`) and let the wrapper stay an internal storage detail.

## Why
A wrapper in a signature is an implementation detail forced onto every caller: they must unwrap it before using the value, and once two crates disagree about which wrapper is required the mismatch cannot always be resolved. Hiding the wrapper behind a plain-type API leaves you free to change the internal storage strategy later without a breaking change.

## Do
```rust
pub struct Config { pub name: String }

pub fn process_data(data: &Config) -> String {
    data.name.clone()
}

pub fn store_config(config: Config) -> Result<(), std::io::Error> {
    let _ = config;
    Ok(())
}
```

## Don't
```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Config { pub name: String }

pub fn process_data(data: Rc<RefCell<Config>>) -> String {
    data.borrow().name.clone()
}
```

## Notes
Wrappers are acceptable in the signature when the type itself is the point of the API (a new smart-pointer or container crate), or when benchmarks show the wrapper is required and the added complexity is a deliberate, documented tradeoff.
