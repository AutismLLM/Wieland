---
id: trait-coherence-newtype
title: Wrap a foreign type in a newtype to implement a foreign trait on it
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, coherence, newtype]
applies_to:
  - "implementing a foreign trait on a foreign type"
  - "orphan rule violation (E0117)"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#trait-coherence-newtype
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Respect the orphan rule: to implement a trait you did not define on a type you did not define, wrap the foreign type in a local newtype and implement the trait on the wrapper.

## Why
Coherence requires that for any `impl Trait for Type`, either the trait or the type is local to the current crate, so two crates can never provide conflicting impls for the same pair. When both the trait (e.g. `Display`) and the type (e.g. `Vec<i32>`) are foreign, the compiler rejects the impl (E0117). A local newtype makes the type local, satisfying the rule; `#[repr(transparent)]` keeps it zero-cost.

## Do
```rust
use std::fmt;

#[repr(transparent)]
struct CommaSeparated(Vec<i32>);

impl CommaSeparated {
    pub fn into_inner(self) -> Vec<i32> { self.0 }
    pub fn inner(&self) -> &Vec<i32> { &self.0 }
}

impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().peekable();
        while let Some(n) = iter.next() {
            write!(f, "{n}")?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl From<Vec<i32>> for CommaSeparated {
    fn from(v: Vec<i32>) -> Self { Self(v) }
}
```

## Don't
```rust
use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
```

## Notes
`#[repr(transparent)]` is mandatory when the wrapper needs the inner type's ABI (FFI, pointer casts); for purely logical wrapping it is optional but good practice. Provide `From`/`Into` and `inner()`/`into_inner()` so callers can move in and out of the wrapper easily.
