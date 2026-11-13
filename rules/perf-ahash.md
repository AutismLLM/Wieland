---
id: perf-ahash
title: Use a faster hasher for maps keyed by trusted data
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, hashing, hashmap]
applies_to:
  - "HashMap keyed by integer IDs or trusted internal handles"
  - "map-heavy hot paths flagged by a profiler"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-ahash
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a map is keyed by trusted internal data and DoS resistance is not needed, swap the default SipHash hasher for `ahash` or `FxHashMap`.

## Why
The standard `HashMap` uses SipHash-1-3, which resists hash-flooding on untrusted input but runs roughly 2-4x slower than non-cryptographic hashers on integer and short-string keys. For maps keyed by compiler-generated IDs, integer handles, or other trusted data the DoS resistance is wasted cost. The wrong choice here is a security bug, not just a slow one: never point a non-DoS-resistant hasher at externally supplied keys.

## Do
```rust
use ahash::AHashMap;

fn build_id_map(ids: &[(u32, String)]) -> AHashMap<u32, String> {
    ids.iter().cloned().collect()
}

use rustc_hash::FxHashMap;

fn build_node_map(nodes: &[(u32, String)]) -> FxHashMap<u32, String> {
    let mut map = FxHashMap::with_capacity_and_hasher(nodes.len(), Default::default());
    map.extend(nodes.iter().cloned());
    map
}
```

## Don't
```rust
use std::collections::HashMap;

fn build_id_map(ids: &[(u32, String)]) -> HashMap<u32, String> {
    ids.iter().cloned().collect()
}
```

## Notes
Profile first: switch hashers only after map operations show up in profiler output. `ahash::AHashMap` uses a random per-process seed and is the safe default upgrade; `FxHashMap` is fastest but predictable, so never expose it to untrusted keys.
