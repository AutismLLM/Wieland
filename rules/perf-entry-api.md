---
id: perf-entry-api
title: Use the entry API for map insert-or-update
category: performance
severity: warn
tier: 2
enforceability: check
tags: [performance, collections, idiomatic]
applies_to:
  - "contains_key followed by insert or get_mut"
  - "HashMap or BTreeMap insert-or-update"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['contains_key', 'get_mut']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-entry-api
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `map.entry(key)` with `or_insert`/`or_insert_with`/`and_modify` for insert-or-update instead of a separate `contains_key`/`get` check followed by `insert`.

## Why
The `contains_key`-then-`insert` pattern hashes the key and walks the table twice — once to check existence, once to mutate. The entry API does a single lookup, computing the hash once and handing back a slot you can fill or modify in place. On `HashMap` and `BTreeMap` it is both faster and more idiomatic, and it avoids the `unwrap()` that the double-lookup form usually needs.

## Do
```rust
use std::collections::HashMap;

fn increment(map: &mut HashMap<String, u32>, key: String) {
    *map.entry(key).or_insert(0) += 1;
}

fn get_or_insert(map: &mut HashMap<String, Vec<i32>>, key: String) -> &mut Vec<i32> {
    map.entry(key).or_insert_with(Vec::new)
}
```

## Don't
```rust
use std::collections::HashMap;

fn increment(map: &mut HashMap<String, u32>, key: String) {
    if map.contains_key(&key) {
        *map.get_mut(&key).unwrap() += 1;
    } else {
        map.insert(key, 1);
    }
}
```

## Notes
For branch-dependent logic, match on `Entry::Occupied` / `Entry::Vacant` directly. Use `.or_default()` for `Default`-constructible values and `.and_modify(f).or_insert(v)` to update-or-insert in one expression.
