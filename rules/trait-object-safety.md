---
id: trait-object-safety
title: Keep a trait dyn-compatible (object-safe) when you need dyn Trait
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, dispatch, dyn-compatibility]
applies_to:
  - "trait used as dyn Trait with a generic method or Self-by-value return"
  - "E0038 the trait cannot be made into an object"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#trait-object-safety
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep a trait dyn-compatible when you need `dyn Trait`: every dispatchable method must go through a vtable, so avoid generic method parameters, bare `Self` by value, and associated constants — or gate those methods with `where Self: Sized`.

## Why
Only dyn-compatible traits can be used as `dyn Trait`. Generic method parameters, `Self`-by-value returns, and associated constants cannot be dispatched through a vtable, and violating the rules produces a hard error (E0038) at the `dyn` use site — often far from the trait definition. Gating the non-dispatchable methods with `where Self: Sized` excludes them from the vtable while keeping the rest of the trait object-safe.

## Do
```rust
trait Transformer {
    fn transform_str(&self, value: &str) -> String;
    fn name(&self) -> &str;

    fn transform_debug<T: std::fmt::Debug>(&self, value: T) -> String
    where
        Self: Sized,
    {
        self.transform_str(&format!("{value:?}"))
    }
}

struct Shout;
impl Transformer for Shout {
    fn transform_str(&self, value: &str) -> String { value.to_uppercase() }
    fn name(&self) -> &str { "shout" }
}

fn apply_all(transformers: &[Box<dyn Transformer>], input: &str) {
    for t in transformers {
        println!("[{}] {}", t.name(), t.transform_str(input));
    }
}
```

## Don't
```rust
trait Transformer {
    fn transform<T: std::fmt::Debug>(&self, value: T) -> String;
    fn name(&self) -> &str;
}

struct Shout;
impl Transformer for Shout {
    fn transform<T: std::fmt::Debug>(&self, value: T) -> String {
        format!("{value:?}").to_uppercase()
    }
    fn name(&self) -> &str { "shout" }
}

fn apply(_t: &dyn Transformer, _x: i32) {}
```

## Notes
`&self`/`&mut self` methods and associated types are fine in a `dyn Trait`. Methods returning `Self` by value, generic method parameters, and associated constants are not — return `Box<Self>` or gate the method with `where Self: Sized` to exclude it from the vtable.
