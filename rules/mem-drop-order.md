---
id: mem-drop-order
title: Order struct fields and locals to control the RAII drop sequence
category: memory
severity: warn
tier: 1
enforceability: judge
tags: [correctness, raii, lifetimes]
applies_to:
  - "struct holding multiple RAII guards (locks, transactions, handles)"
  - "MutexGuard alongside a resource it protects"
  - "explicit drop to sequence cleanup"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-drop-order
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When multiple RAII guards must be released in a specific order, encode it: struct fields drop top-to-bottom, locals drop in reverse declaration order, and use an explicit `drop` when neither matches the needed sequence.

## Why
Drop order is observable and deterministic, and RAII guards do real work in `Drop` — releasing locks, committing transactions, closing handles. Dropping them in the wrong order silently breaks invariants, such as releasing a mutex while a transaction it protects is still in flight. The rules are fixed but easy to overlook as fields and locals accumulate, so the declaration order must be treated as the contract.

## Do
```rust
use std::sync::MutexGuard;

struct Transaction;

impl Drop for Transaction {
    fn drop(&mut self) {
        println!("transaction committed");
    }
}

struct DatabaseSession {
    transaction: Transaction,
    guard: MutexGuard<'static, ()>,
}
```

## Don't
```rust
use std::sync::MutexGuard;

struct Transaction;

impl Drop for Transaction {
    fn drop(&mut self) {
        println!("transaction committed");
    }
}

struct DatabaseSession {
    guard: MutexGuard<'static, ()>,
    transaction: Transaction,
}
```

## Notes
When the natural scope order is wrong for locals, call `drop(guard)` explicitly rather than reordering declarations. Use `std::mem::ManuallyDrop` only when you must opt out of automatic drop entirely, and remember `std::mem::forget` runs no destructor at all.
