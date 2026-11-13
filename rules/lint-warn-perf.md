---
id: lint-warn-perf
title: Enable clippy::perf to catch obvious allocation and copy waste
category: lints
severity: warn
tier: 2
enforceability: gate
tags: [clippy, performance, allocation]
applies_to:
  - "single-char patterns passed as &str"
  - "redundant to_owned / to_string before a move"
  - "manual collection initialization"
rust_edition: "2024"
msrv: null
detect:
  clippy: [single_char_pattern, unnecessary_to_owned, box_collection, redundant_allocation, slow_vector_initialization, large_enum_variant]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-warn-perf
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Enable the `clippy::perf` group so obvious performance anti-patterns — needless allocations, redundant copies, `&str` where a `char` would do — are flagged as you write them.

## Why
Not every inefficiency matters, but the ones `clippy::perf` catches are free wins with no downside: `s.contains("x")` does a substring search where `s.contains('x')` does a byte scan, `.iter().cloned().collect()` reallocates where `.to_vec()` does not, and a huge enum variant bloats every value of the enum. Fixing them costs nothing and removes waste before it compounds in a hot path.

## Do
```rust
fn process(input: &str) -> String {
    input
        .split(',')
        .filter(|part| part.starts_with(' '))
        .map(str::trim)
        .collect()
}
```

## Don't
```rust
fn process(input: &str) -> String {
    let parts: Vec<_> = input.split(",").collect();
    let mut result = String::new();
    for part in parts.iter() {
        if part.starts_with(" ") {
            result = result + &part.trim().to_string();
        }
    }
    result
}
```

## Notes
`clippy::perf` catches obvious inefficiencies, not algorithmic ones — it will not tell you an `O(n^2)` loop should be a hash join. Profile before restructuring; use these lints as hygiene, not as a substitute for measurement.
