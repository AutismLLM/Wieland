---
id: own-rc-single-thread
title: Use Rc<T> for shared ownership in single-threaded code
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [rc, shared-ownership, single-thread]
applies_to:
  - "graph or tree with shared nodes on one thread"
  - "Arc used where no thread boundary is crossed"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-rc-single-thread
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For shared ownership that never crosses a thread boundary, use `Rc<T>` rather than `Arc<T>` to avoid the cost of atomic reference counting.

## Why
`Rc` increments and decrements its count with plain arithmetic; `Arc` uses atomic operations that carry measurable overhead even without contention. In confirmed single-threaded code that overhead buys nothing. `Rc` is `!Send + !Sync`, so the compiler enforces the single-thread assumption for you and will point you to `Arc` the moment you try to send it across threads.

## Do
```rust
use std::rc::Rc;

let root = Rc::new(Node::new("root"));
let child = Rc::new(Node::new("child"));
root.add_child(Rc::clone(&child));
```

## Don't
```rust
use std::sync::Arc;

let root = Arc::new(Node::new("root"));
let child = Arc::new(Node::new("child"));
root.add_child(Arc::clone(&child));
```

## Notes
Prefer `Rc::clone(&x)` over `x.clone()` to make the cheap refcount bump visually distinct from a deep clone. Break reference cycles with `Weak<T>` for back-references, and pair `Rc` with `RefCell` (`Rc<RefCell<T>>`) when you also need shared mutation.
