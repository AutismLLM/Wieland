---
id: name-variants-camel
title: Use UpperCamelCase for enum variants
category: naming
severity: warn
tier: 1
enforceability: judge
tags: [naming, enums, casing]
applies_to:
  - "enum variant declarations"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-variants-camel
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name every enum variant in `UpperCamelCase`.

## Why
Variants follow the same convention as types, so `UpperCamelCase` keeps them distinct from fields, variables, and functions and lets a reader recognize a variant on sight. The compiler warns on lowercase or screaming-case variants, so any other casing is both non-idiomatic and noisy.

## Do
```rust
enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
}
```

## Don't
```rust
enum Status {
    pending,
    in_progress,
    COMPLETED,
}
```

## Notes
Keep variant names specific and free of the enum's own name: prefer `Connected` over `ConnectionConnected` inside `ConnectionState`.
