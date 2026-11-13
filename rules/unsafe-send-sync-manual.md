---
id: unsafe-send-sync-manual
title: Document invariants for manual Send/Sync impls; prefer letting the compiler derive
category: unsafe
severity: warn
tier: 2
enforceability: check
tags: [concurrency, auto-traits, data-race]
applies_to:
  - "unsafe impl Send"
  - "unsafe impl Sync"
  - "types wrapping raw pointers or Cell"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['unsafe impl (Send|Sync)', 'unsafe impl<.*> (Send|Sync)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-send-sync-manual
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Document the invariants when manually implementing `Send` or `Sync`; prefer letting the compiler derive them automatically.

## Why
`Send` and `Sync` are unsafe auto-traits the compiler derives correctly for most types. A manual `unsafe impl` signals that the borrow checker cannot verify the invariant itself — get it wrong and you introduce data races unreachable by safe code. Worse, an undocumented manual impl is load-bearing: the next person to change the type's fields may silently break the invariant without realizing the impl depends on it.

## Do
```rust
use std::sync::Mutex;

/// A buffer that owns its heap allocation exclusively (no aliasing).
struct OwnedBuffer {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: OwnedBuffer owns its allocation exclusively; no other reference to
// that allocation exists, and the pointer is valid for its entire lifetime.
unsafe impl Send for OwnedBuffer {}

// SAFETY: all mutating methods require &mut self, so concurrent & references
// cannot mutate the buffer.
unsafe impl Sync for OwnedBuffer {}

fn make_shared() -> std::sync::Arc<Mutex<u32>> {
    // Arc<Mutex<u32>> is Send + Sync automatically — no manual impl required.
    std::sync::Arc::new(Mutex::new(0))
}
```

## Don't
```rust
use std::cell::Cell;

struct SharedCounter {
    value: Cell<u32>,
}

unsafe impl Sync for SharedCounter {}
unsafe impl Send for SharedCounter {}
```

## Notes
Reach for manual impls only when raw pointers, `Cell`, or other non-auto types are involved. `PhantomData<*const T>` is the canonical way to make a type `!Send + !Sync` without any `unsafe impl`. Adding `unsafe impl Send` to a type containing `Rc<T>` or `Cell<T>` is almost certainly unsound — prefer `Arc<T>` and `Mutex<T>`.
