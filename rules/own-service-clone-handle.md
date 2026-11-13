---
id: own-service-clone-handle
title: Make heavyweight service types Clone via a shared Arc<Inner> handle
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [ownership, arc, api-design]
applies_to:
  - "a service or client type instantiated once per thread and passed to many handlers"
  - "struct holding heavyweight state without Clone"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SERVICES-CLONE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give heavyweight service or client types a cheap, shared-ownership `Clone` by storing their actual state in an `Arc<Inner>` and deriving `Clone` on the outer handle, so every consumer can hold its own handle to the same underlying instance.

## Why
Once two handlers on the same thread each need the same underlying service, something has to own it — and if the service itself is not cheaply cloneable, every caller either fights the borrow checker or reaches for its own ad hoc `Rc`/`Arc` wrapper, which then leaks into the type's public signature. Wrapping the real state in `Arc<Inner>` and deriving `Clone` on the outer type gives every consumer a cheap handle to shared state without ever changing the type callers write down.

## Do
```rust
use std::sync::Arc;

struct ServiceCommonInner {
    name: String,
}

#[derive(Clone)]
pub struct ServiceCommon {
    inner: Arc<ServiceCommonInner>,
}

impl ServiceCommon {
    pub fn new(name: String) -> Self {
        Self { inner: Arc::new(ServiceCommonInner { name }) }
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }
}

struct ServiceA {
    common: ServiceCommon,
}

impl ServiceA {
    fn new(common: &ServiceCommon) -> Self {
        Self { common: common.clone() }
    }
}
```

## Don't
```rust
struct ServiceCommon {
    name: String,
}

// Forces every caller to wrap ServiceCommon in Arc/Rc itself, or
// duplicate state when passing it to two handlers on the same thread.
struct ServiceA {
    common: ServiceCommon,
}
```

## Notes
The `Clone` here must stay a cheap pointer-copy, never a deep copy of the service's data — that is the entire point of the `Arc<Inner>` split. Combine with keeping the wrapper itself out of the public signature: callers see `ServiceCommon`, never `Arc<ServiceCommonInner>` directly.
