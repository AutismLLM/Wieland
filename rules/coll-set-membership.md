---
id: coll-set-membership
title: Use HashSet/BTreeSet for membership and dedup, not linear Vec::contains
category: collections
severity: warn
tier: 2
enforceability: check
tags: [performance, data-structures, sets]
applies_to:
  - "Vec::contains inside a loop"
  - "deduplicating by scanning an accumulating Vec"
  - "repeated membership tests"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['\.contains\(', 'if !seen\.contains']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#coll-set-membership
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For repeated membership tests or deduplication, use `HashSet`/`BTreeSet` instead of linear `Vec::contains`.

## Why
`Vec::contains` is O(n) per call. Testing m items against a list of n in a loop is O(n * m) — quadratic. A `HashSet` cuts each check to O(1) average, making the loop O(n + m); use `BTreeSet` when you also need sorted iteration. Deduplication is the same story: collecting into a set is one line and O(n), while scanning a growing `Vec` for each item is quadratic. Keep a `Vec` only when order matters, duplicates are intentional, or the collection is tiny (say <= 8 items).

## Do
```rust
use std::collections::{BTreeSet, HashSet};

fn find_common(all_users: &[String], active_ids: &[String]) -> Vec<String> {
    let active: HashSet<&String> = active_ids.iter().collect();
    all_users
        .iter()
        .filter(|u| active.contains(u))
        .cloned()
        .collect()
}

fn deduplicate_ordered(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::with_capacity(items.len());
    items.into_iter().filter(|s| seen.insert(s.clone())).collect()
}

fn unique_sorted(items: Vec<String>) -> Vec<String> {
    items.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
}
```

## Don't
```rust
fn find_common(all_users: &[String], active_ids: &[String]) -> Vec<String> {
    let mut common = Vec::new();
    for user in all_users {
        if active_ids.contains(user) {
            common.push(user.clone());
        }
    }
    common
}

fn deduplicate(items: Vec<String>) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for item in items {
        if !seen.contains(&item) {
            seen.push(item);
        }
    }
    seen
}
```

## Notes
When a set will be queried many times, call `HashSet::with_capacity(n)` up front to avoid rehashing.
