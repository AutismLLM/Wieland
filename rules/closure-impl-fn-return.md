---
id: closure-impl-fn-return
title: Return closures as impl Fn instead of Box<dyn Fn>
category: closures
severity: warn
tier: 2
enforceability: check
tags: [return-position, static-dispatch, allocation]
applies_to:
  - "function returning Box<dyn Fn>"
  - "closure returned from a function"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['->\s*Box<dyn\s+Fn', '->\s*Box<dyn\s+FnMut', '->\s*Box<dyn\s+FnOnce']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#closure-impl-fn-return
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Return a closure as `impl Fn`/`FnMut`/`FnOnce` for zero-allocation static dispatch, and reserve `Box<dyn Fn>` for when the return type genuinely varies.

## Why
`impl Fn` in return position names the closure's concrete but unnameable type, so the call is statically dispatched with no heap allocation. `Box<dyn Fn>` adds an allocation at construction and a virtual call on every invocation. The `impl Trait` syntax exists precisely for this case; boxing a single concrete closure pays for indirection you do not use.

## Do
```rust
fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn counter(start: i32) -> impl FnMut() -> i32 {
    let mut n = start;
    move || {
        let current = n;
        n += 1;
        current
    }
}

fn demo() {
    let add5 = adder(5);
    assert_eq!(add5(10), 15);

    let mut next = counter(0);
    assert_eq!(next(), 0);
    assert_eq!(next(), 1);
}
```

## Don't
```rust
fn adder_bad(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
```

## Notes
`Box<dyn Fn>` is required when different `if`/`match` arms return distinct closure types that `impl Fn` cannot unify, or when the closure is stored in a struct field or collection (`Vec<Box<dyn Fn(i32) -> i32>>`).
