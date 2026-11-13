---
id: unsafe-ffi-dll-portable-state
title: Only share portable, repr(C) data across a Rust DLL boundary
category: unsafe
severity: warn
tier: 1
enforceability: judge
tags: [ffi, dll, abi, layout]
applies_to:
  - "data passed between two independently-compiled Rust dynamic libraries"
  - "a type shared through an FFI function signature between DLLs"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-ISOLATE-DLL-STATE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When two or more Rust-based dynamic libraries are loaded in one process, only exchange "portable" data between them: `#[repr(C)]` (or otherwise well-defined layout), with no interaction with any `static`, thread-local, or `TypeId`, and no pointer or reference into non-portable data.

## Why
The compiler treats each DLL as its own compilation artifact: every DLL gets its own copy of `static`/thread-local storage, its own `TypeId` universe, and its own layout decision for any `#[repr(Rust)]` (default) type. Passing a `String`, `Vec<u8>`, `Box<T>`, or a `#[repr(Rust)]` struct across the boundary — or calling into a library like `tokio` or `log` that leans on its own statics from the wrong side — silently corrupts state or invokes undefined behavior, because the receiving DLL has no matching allocator bookkeeping, layout, or global state for what it receives.

## Do
```rust
#[repr(C)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[unsafe(no_mangle)]
pub extern "C" fn distance(a: Point3, b: Point3) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
```

## Don't
```rust
pub struct CommonService {
    cache: std::collections::HashMap<u64, Vec<u8>>,
}

#[unsafe(no_mangle)]
pub extern "C" fn use_common_service(common: &CommonService) {
    // `CommonService` is #[repr(Rust)]: its layout can differ between
    // the DLL that allocated it and the DLL reading it here.
    // Any method call through it also runs with THIS DLL's statics,
    // not the allocating DLL's — silent corruption or UB.
    common.cache.len();
}
```

## Notes
This also covers types nested arbitrarily deep — a portable outer struct that stores a reference into a non-portable inner value is fine, but one that stores the non-portable value itself, or a `TypeId`-based dispatch table, is not. When two DLLs must share rich state, do it through a stable, versioned `#[repr(C)]` vtable or a serialization boundary, never by handing over native Rust types.
</content>
