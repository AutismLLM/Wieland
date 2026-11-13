---
id: lint-warn-suspicious
title: Enable clippy::suspicious to surface likely bugs early
category: lints
severity: warn
tier: 2
enforceability: gate
tags: [clippy, suspicious, bugs, review]
applies_to:
  - "side effects inside .map()"
  - "confusing operator precedence in bit/shift expressions"
  - "impossible comparison ranges"
rust_edition: "2024"
msrv: null
detect:
  clippy: [suspicious_map, suspicious_splitn, suspicious_else_formatting, suspicious_op_assign_impl, suspicious_arithmetic_impl]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-warn-suspicious
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Enable the `clippy::suspicious` group so code that is valid syntax but almost certainly a mistake gets flagged for a second look.

## Why
Suspicious lints catch the patterns that compile fine yet are usually typos or precedence traps: a `.map()` that only performs side effects, `1 << 4 + 1` where the author meant `(1 << 4) + 1`, or a `splitn` that can never produce its `n` parts. These are latent bugs; warning on them turns a silent wrong answer into a prompt to check the intent.

## Do
```rust
fn log_all(items: &[u32]) {
    items.iter().for_each(|x| println!("{x}"));
}
```

## Don't
```rust
fn log_all(items: &[u32]) {
    let _: Vec<_> = items
        .iter()
        .map(|x| {
            println!("{x}");
            x
        })
        .collect();
}
```

## Notes
Suspicious lints rarely need suppressing. When a construct is deliberate — for example a custom arithmetic-trait impl that legitimately uses `+` in a reduction step — `#[allow]` the specific lint on the specific impl and note why.
