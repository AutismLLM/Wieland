---
id: async-fn-in-trait
title: Use native async fn in traits instead of the async_trait macro
category: async
severity: info
tier: 2
enforceability: check
tags: [traits, dependencies, allocation]
applies_to:
  - "#[async_trait] attribute"
  - "async fn declared in a trait"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['async_trait']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-fn-in-trait
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write `async fn` directly in trait definitions (stable since 1.75) instead of pulling in `#[async_trait]`, unless you specifically need `dyn` dispatch.

## Why
Native async fn in traits removes the `async_trait` proc-macro dependency and the hidden `Box<dyn Future>` heap allocation it inserts on every call. That means fewer allocations, no macro expansion, and one less crate to audit. The tradeoff is two caveats: native AFIT is not yet `dyn`-compatible, and its futures are not `Send` by default.

## Do
```rust
trait Repo {
    async fn get(&self, id: u64) -> anyhow::Result<String>;
    async fn save(&self, value: String) -> anyhow::Result<()>;
}

struct PgRepo;

impl Repo for PgRepo {
    async fn get(&self, id: u64) -> anyhow::Result<String> {
        Ok(format!("row-{id}"))
    }

    async fn save(&self, value: String) -> anyhow::Result<()> {
        let _ = value;
        Ok(())
    }
}
```

## Don't
```rust
use async_trait::async_trait;

#[async_trait]
trait Repo {
    async fn get(&self, id: u64) -> anyhow::Result<String>;
    async fn save(&self, value: String) -> anyhow::Result<()>;
}
```

## Notes
For `Box<dyn Trait>` keep `#[async_trait]` or use `trait-variant`. For spawned tasks on a multi-threaded runtime you need `Send`: use `#[trait_variant::make(TraitSend: Send)]` or bound the return with `-> impl Future<Output = ...> + Send`.
