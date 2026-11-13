---
id: name-type-param-single
title: Use single uppercase letters for type parameters
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, generics, type-parameters]
applies_to:
  - "generic type parameter lists"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-type-param-single
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name generic type parameters with single uppercase letters (`T`, `E`, `K`, `V`), moving any complexity into a `where` clause rather than the parameter name.

## Why
Single-letter parameters keep signatures compact, and the conventional letters — `T` type, `E` error, `K` key, `V` value — are instantly recognized from `Result<T, E>`, `HashMap<K, V>`, and `Option<T>`. A verbose `ElementType` bloats the signature without adding meaning that a short name and clear bounds do not already convey.

## Do
```rust
struct Container<T> {
    items: Vec<T>,
}

fn process<I, O>(input: I) -> O
where
    I: Into<O>,
{
    input.into()
}
```

## Don't
```rust
struct Container<ElementType> {
    items: Vec<ElementType>,
}

fn process<InputType, OutputType>(input: InputType) -> OutputType
where
    InputType: Into<OutputType>,
{
    input.into()
}
```

## Notes
When several parameters genuinely need clarification, a short descriptive name is acceptable (`struct Query<Db, Row, Err>`), but keep bounds in the `where` clause.
