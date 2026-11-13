---
id: proj-pub-crate-internal
title: Use pub(crate) for internal APIs to keep the public surface small
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [visibility, encapsulation, api-stability]
applies_to:
  - "helper types and functions marked pub but only used inside the crate"
  - "internal modules exposed as pub"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-pub-crate-internal
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Mark items that are shared inside the crate but not part of the public API as `pub(crate)` rather than `pub`.

## Why
`pub(crate)` exposes an item across your crate while hiding it from external users, drawing a clear line between public API and implementation detail. Making internals `pub` lets downstream code depend on them, so any refactor becomes a breaking change; `pub(crate)` lets you rebuild the internals freely because only the truly `pub` items form your compatibility contract.

## Do
```rust
pub(crate) mod internal {
    pub(crate) struct InternalState {
        pub(crate) buffer: Vec<u8>,
        pub(crate) dirty: bool,
    }
    pub(crate) fn process(state: &mut InternalState) {
        state.dirty = false;
    }
}

pub struct Widget {
    state: internal::InternalState,
}
```

## Don't
```rust
// everything public — users can now depend on implementation details
pub mod internal {
    pub struct InternalState {
        pub buffer: Vec<u8>,
        pub dirty: bool,
    }
    pub fn process(state: &mut InternalState) {
        state.dirty = false;
    }
}
```

## Notes
Reach for narrower scopes when they fit: `pub(super)` for helpers shared only with a parent module, plain private for a single module's own details, and re-export only the intended API with `pub use`.
