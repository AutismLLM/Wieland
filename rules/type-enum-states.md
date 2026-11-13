---
id: type-enum-states
title: Model mutually exclusive states with an enum, not boolean flags
category: types
severity: warn
tier: 0
enforceability: gate
tags: [enums, state, invariants]
applies_to:
  - "struct with several bool fields"
  - "value that is in exactly one of N states"
rust_edition: "2024"
msrv: null
detect:
  clippy: [struct_excessive_bools]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-enum-states
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a value can be in exactly one of several states, encode it as an enum so invalid combinations are unrepresentable.

## Why
Boolean flags and parallel optional fields can express impossible combinations — `is_running && is_completed`, or a `socket` that is `None` while `is_connected` is true. An enum gives each state exactly the data it needs and makes `match` exhaustive, so the compiler forces every case to be handled and rejects contradictory states outright.

## Do
```rust
use std::time::Instant;

enum TaskState {
    Pending,
    Running { started_at: Instant },
    Completed { result: Output },
    Failed { error: Error },
}

struct Task {
    state: TaskState,
}
```

## Don't
```rust
struct Task {
    is_running: bool,
    is_completed: bool,
    is_failed: bool,
    error: Option<Error>,
}
```

## Notes
`Option<T>` and `Result<T, E>` are the standard-library instances of this pattern — reach for them instead of nullable or sentinel values. `std::mem::replace` lets you move data between states in a `&mut self` transition without cloning.
