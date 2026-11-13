---
id: closure-static-vs-dyn
title: Take impl Fn for hot callbacks; use dyn Fn to cut code size or to store them
category: closures
severity: info
tier: 1
enforceability: judge
tags: [dispatch, monomorphization, code-size]
applies_to:
  - "generic F: Fn callback parameter"
  - "closures stored in a struct or collection"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#closure-static-vs-dyn
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Accept `impl Fn`/generic `F: Fn` for hot, inlinable callbacks, and switch to `&dyn Fn`/`Box<dyn Fn>` to cut binary size or to store heterogeneous closures.

## Why
A generic `F: Fn` monomorphizes at each call site, emitting a specialized copy that can inline for zero-cost dispatch — at the cost of binary bloat when many closure types are substituted. `Box<dyn Fn>` shares one compiled copy through a vtable, which reduces code size and is the only way to store mixed closures (an event-handler registry). Choose by the concrete requirement, not by habit.

## Do
```rust
fn transform<F: Fn(i32) -> i32>(xs: &[i32], f: F) -> Vec<i32> {
    xs.iter().map(|&x| f(x)).collect()
}

struct Registry {
    handlers: Vec<Box<dyn Fn(&str)>>,
}

impl Registry {
    fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    fn register(&mut self, handler: impl Fn(&str) + 'static) {
        self.handlers.push(Box::new(handler));
    }

    fn dispatch(&self, event: &str) {
        for handler in &self.handlers {
            handler(event);
        }
    }
}

fn demo() {
    let doubled = transform(&[1, 2, 3], |x| x * 2);
    assert_eq!(doubled, vec![2, 4, 6]);

    let mut reg = Registry::new();
    reg.register(|e| println!("logger: {e}"));
    reg.dispatch("user_signup");
}
```

## Don't
```rust
struct BadRegistry<F: Fn(&str)> {
    handler: F,
}

fn transform_slow(xs: &[i32], f: &dyn Fn(i32) -> i32) -> Vec<i32> {
    xs.iter().map(|&x| f(x)).collect()
}
```

## Notes
`&dyn Fn` avoids an allocation when you only borrow a closure for one call and do not store it — pass `&closure` rather than boxing. Callbacks called across an `await` point need `Box<dyn Fn + Send>`.
