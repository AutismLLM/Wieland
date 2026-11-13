---
id: mem-take-replace
title: Use mem::take or mem::replace to move out of a &mut without cloning
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, ownership, allocation]
applies_to:
  - "cloning a field just to drain or reset it"
  - "moving a value out of &mut self"
  - "state-machine transition or Drop that consumes a field"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-take-replace
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
To move a value out of a `&mut` reference, use `mem::take` (leaving `Default::default()`) or `mem::replace` (leaving an explicit value) instead of cloning.

## Why
Ownership rules forbid moving a field out of `&mut self` because the field cannot be left invalid, and the reflexive workaround is `.clone()` — which allocates for nothing. `mem::take` swaps the field with its default and hands back the original with zero copies; `mem::replace` does the same with a value you choose. Both are inlined down to a few register moves.

## Do
```rust
use std::mem;

struct Processor {
    items: Vec<String>,
}

impl Processor {
    fn flush(&mut self) -> Vec<String> {
        mem::take(&mut self.items)
    }
}
```

## Don't
```rust
struct Processor {
    items: Vec<String>,
}

impl Processor {
    fn flush(&mut self) -> Vec<String> {
        let v = self.items.clone();
        self.items.clear();
        v
    }
}
```

## Notes
`mem::take` needs `T: Default`. When there is no meaningful default, use `mem::replace` with an explicit next value — common in state machines — or hold the value in `Option<T>` and call `.take()`, which yields `None`.
