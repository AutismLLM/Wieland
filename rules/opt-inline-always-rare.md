---
id: opt-inline-always-rare
title: Reserve #[inline(always)] for tiny hot functions proven by profiling
category: optimization
severity: warn
tier: 2
enforceability: check
tags: [performance, inlining, attributes]
applies_to:
  - "#[inline(always)] on non-trivial or non-hot functions"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['#\[inline\(always\)\]']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-inline-always-rare
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `#[inline(always)]` only on tiny functions in measured hot inner loops; let the compiler's heuristics decide everywhere else.

## Why
`#[inline(always)]` overrides the compiler's inlining heuristics, which are usually better informed than intuition. Sprinkling it across accessors and ordinary helpers bloats the binary, evicts hot code from the instruction cache, and can slow the program down. Reserve it for small, extremely hot functions where a benchmark shows a real win.

## Do
```rust
impl Hasher for MyHasher {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.state = self.state.wrapping_add(bytes.len() as u64);
    }
}
```

## Don't
```rust
#[inline(always)]
pub fn get_name(&self) -> &str {
    &self.name
}

#[inline(always)]
pub fn calculate_tax(amount: f64) -> f64 {
    amount * 0.1
}
```

## Notes
For ordinary small functions or cross-crate generics, plain `#[inline]` is the right hint — it suggests inlining without forcing it. Confirm the win with `cargo bloat` and `cargo asm` before keeping `always`.
