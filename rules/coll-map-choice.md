---
id: coll-map-choice
title: Pick the map by access pattern (HashMap, BTreeMap, or IndexMap)
category: collections
severity: info
tier: 1
enforceability: judge
tags: [performance, data-structures, maps]
applies_to:
  - "choosing a key-value collection"
  - "needing sorted iteration or range queries"
  - "needing deterministic, insertion-order output"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#coll-map-choice
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Choose the map by its access pattern: `HashMap` for fast unordered lookup, `BTreeMap` for sorted iteration and range queries, `IndexMap` for insertion-order iteration with O(1) lookup.

## Why
`HashMap` is the right default: O(1) average lookup and insertion with no ordering overhead. `BTreeMap` keeps keys sorted in a B-tree, enabling range queries and ordered iteration at O(log n) cost. `IndexMap` (from the `indexmap` crate) preserves insertion order with O(1) average lookup, which matters for deterministic output, config files, or any API where output order must mirror input order. Picking the wrong map either wastes CPU on ordering you do not need or forces you to sort after the fact.

## Do
```rust
use std::collections::BTreeMap;
use indexmap::IndexMap;

fn events_in_range(
    log: &BTreeMap<u64, String>,
    start: u64,
    end: u64,
) -> Vec<(&u64, &String)> {
    log.range(start..=end).collect()
}

fn parse_config(pairs: &[(&str, &str)]) -> IndexMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}
```

## Don't
```rust
use std::collections::HashMap;

fn word_counts(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn report(text: &str) {
    for (word, count) in &word_counts(text) {
        println!("{word}: {count}");
    }
}
```

## Notes
The default `HashMap` uses SipHash-1-3, which is DoS-resistant but not the fastest. For non-adversarial hot paths, swap in a faster hasher as a type parameter. For a tiny map (<= 8 entries) an array of tuples can beat any of them.
