---
id: mem-compact-string
title: Use a compact string type for large numbers of short strings
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, strings]
applies_to:
  - "struct field holding many short String values"
  - "compact_str"
  - "millions of short strings in memory"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-compact-string
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When storing millions of short strings, use a compact string type such as `CompactString` that keeps small values inline instead of `String`.

## Why
A `String` always heap-allocates and its 24-byte header points to a separate allocation, so millions of short strings pay both the header cost and an allocation each. `CompactString` (and similar types) store strings up to 23 bytes inline with no heap allocation, keeping the same footprint while eliminating most allocations. Fewer allocations mean less allocator pressure and better cache behavior.

## Do
```rust
use compact_str::CompactString;

struct User {
    id: u64,
    username: CompactString,
    email: CompactString,
}
```

## Don't
```rust
struct User {
    id: u64,
    username: String,
    email: String,
}
```

## Notes
Prefer plain `String` for hot-path string manipulation (it is optimized for mutation) and at public API boundaries, where `impl Into<String>` avoids forcing the dependency on callers.
