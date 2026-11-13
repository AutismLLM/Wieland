---
id: own-refcell-interior
title: Use RefCell<T> for interior mutability in single-threaded code
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [refcell, interior-mutability, single-thread]
applies_to:
  - "cache or lazy field mutated through &self"
  - "shared single-threaded state via Rc<RefCell<T>>"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-refcell-interior
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `RefCell<T>` to mutate data through a shared `&self` reference in single-threaded code, moving borrow checking from compile time to runtime.

## Why
The borrow checker forbids mutation through `&self`, which blocks legitimate patterns like caches and lazy initialization where the mutation is logically internal. `RefCell` enforces the same aliasing rules dynamically, panicking only if you actually hold conflicting borrows at runtime. For plain `Copy` values `Cell<T>` is even lighter — `get`/`set` with no borrow flags and no panic path.

## Do
```rust
use std::cell::RefCell;
use std::collections::HashMap;

struct Cache {
    data: RefCell<HashMap<String, String>>,
}

impl Cache {
    fn get_or_compute(&self, key: &str) -> String {
        let mut data = self.data.borrow_mut();
        data.entry(key.to_string())
            .or_insert_with(|| expensive_compute(key))
            .clone()
    }
}
```

## Don't
```rust
use std::collections::HashMap;

struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    fn get_or_compute(&mut self, key: &str) -> String {
        self.data
            .entry(key.to_string())
            .or_insert_with(|| expensive_compute(key))
            .clone()
    }
}
```

## Notes
`RefCell` panics on a runtime borrow violation — use `try_borrow`/`try_borrow_mut` where a conflict is possible and recoverable. It is `!Sync`; reach for `Mutex`/`RwLock` once threads are involved.
