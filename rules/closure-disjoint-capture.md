---
id: closure-disjoint-capture
title: Capture only the fields a closure uses; add move only when ownership is required
category: closures
severity: info
tier: 1
enforceability: judge
tags: [captures, edition-2021, borrowing]
applies_to:
  - "closure capturing a struct field"
  - "move closure inside a method"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#closure-disjoint-capture
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write closures that reference only the specific fields they need, and reach for `move` on a whole named place only when ownership is genuinely required.

## Why
Since Rust 2021, closures capture individual fields (`config.threshold`) rather than the whole struct, so sibling fields stay independently accessible. Writing `move || self.field` moves the entire `*self`, not just the field, which needlessly blocks the rest of the struct. Binding the one field you need to a local first narrows the capture and keeps the surrounding value usable.

## Do
```rust
struct Config {
    threshold: i32,
    label: String,
}

fn make_checker(config: Config) -> (impl Fn() -> bool, String) {
    let threshold = config.threshold;
    let checker = move || threshold > 0;
    (checker, config.label)
}

fn demo() {
    let cfg = Config { threshold: 5, label: String::from("info") };
    let (check, label) = make_checker(cfg);
    println!("label returned: {label}");
    assert!(check());
}
```

## Don't
```rust
struct Config {
    threshold: i32,
    label: String,
}

fn make_checker(config: Config) -> impl Fn() -> bool {
    let checker = move || config.threshold > 0;
    checker
}
```

## Notes
`Copy` fields (integers, booleans) are copied into a `move` closure, so the original stays valid. Only escalate from borrowing to `move` when the closure must outlive the enclosing scope.
