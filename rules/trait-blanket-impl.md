---
id: trait-blanket-impl
title: Use a blanket impl to give behaviour to every type that satisfies a bound
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, generics, coherence]
applies_to:
  - "extension trait with identical logic across many types"
  - "impl<T: Bound> Trait for T"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#trait-blanket-impl
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use a blanket impl `impl<T: Bound> Trait for T` to give behaviour to every type that satisfies a bound instead of repeating the same impl per type.

## Why
A blanket impl extends a whole class of types at once: the standard library blanket-impls `ToString` for every `T: Display`, so any `Display` type gets `.to_string()` for free. This removes boilerplate and keeps extension traits composable. The trade-off is coherence — Rust allows at most one applicable impl per type, so a blanket impl can never coexist with a specific one (E0119), and adding a public blanket impl is at least a minor semver change.

## Do
```rust
use std::fmt;

trait Describe {
    fn describe(&self) -> String;
}

impl<T: fmt::Display> Describe for T {
    fn describe(&self) -> String {
        format!("{} ({})", self, std::any::type_name::<T>())
    }
}

fn print_described(value: &impl Describe) {
    println!("{}", value.describe());
}
```

## Don't
```rust
trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 {
    fn describe(&self) -> String { format!("i32: {self}") }
}
impl Describe for f64 {
    fn describe(&self) -> String { format!("f64: {self}") }
}
impl Describe for bool {
    fn describe(&self) -> String { format!("bool: {self}") }
}
```

## Notes
A blanket impl prevents any specific impl for a type it already covers, because specialization is not stable. If you need a per-type override, wrap that type in a newtype instead. Pair blanket impls with sealed traits when you want the behaviour but must prevent external implementations.
