---
id: trait-default-methods
title: Define a trait as a few required methods plus defaulted ones built on top
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, defaults, ergonomics]
applies_to:
  - "trait where several methods are mechanical compositions of others"
  - "trait forcing implementors to write derivable behaviour"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#trait-default-methods
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Define a trait in terms of a small set of required methods plus defaulted methods built on top of them, so implementors write only the essential logic.

## Why
A trait with only required methods puts the full burden on every consumer. Default method bodies — as `Iterator` does, building dozens of combinators on a single required `next` — let implementors provide only the core and receive the rest for free. Defaults also document the canonical relationship between methods, and implementors can still override one for performance without changing the observable contract.

## Do
```rust
trait Summarise {
    fn sentences(&self) -> Vec<String>;

    fn first_sentence(&self) -> Option<String> {
        self.sentences().into_iter().next()
    }

    fn word_count(&self) -> usize {
        self.sentences().join(" ").split_whitespace().count()
    }

    fn is_empty(&self) -> bool {
        self.sentences().is_empty()
    }
}

struct Article { body: String }

impl Summarise for Article {
    fn sentences(&self) -> Vec<String> {
        self.body
            .split('.')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect()
    }
}
```

## Don't
```rust
trait Summarise {
    fn sentences(&self) -> Vec<String>;
    fn first_sentence(&self) -> Option<String>;
    fn word_count(&self) -> usize;
}

struct Article { body: String }

impl Summarise for Article {
    fn sentences(&self) -> Vec<String> {
        self.body.split('.').map(str::trim).map(str::to_owned).collect()
    }
    fn first_sentence(&self) -> Option<String> {
        self.sentences().into_iter().next()
    }
    fn word_count(&self) -> usize {
        self.sentences().join(" ").split_whitespace().count()
    }
}
```

## Notes
Keep required methods to the minimum orthogonal set, ideally one or two. Default bodies must only call other methods on `Self`, not external state. An override may change performance but must preserve the same observable semantics.
