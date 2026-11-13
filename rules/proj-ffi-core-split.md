---
id: proj-ffi-core-split
title: Keep business logic in the core crate; the FFI crate only translates
category: project-layout
severity: warn
tier: 1
enforceability: judge
tags: [ffi, architecture, separation-of-concerns]
applies_to:
  - "a crate pair split into `foo` (core) and `foo-ffi` (C ABI glue)"
  - "extern \"C\" functions that also contain business logic"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-FFI-TRANSLATES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a core crate `foo` is exposed to other languages through a glue crate `foo-ffi`, all operational logic stays in `foo` as idiomatic, safe, testable Rust. `foo-ffi` only translates between Rust and C constructs — it must not grow business logic of its own, even at the cost of repeating and slightly adjusting a signature.

## Why
`foo-ffi` types are constrained by C ABI concerns — `#[repr(C)]`, raw pointers, `extern "C"` signatures — which are a poor fit for expressing and testing domain logic. Letting interop concerns leak into `foo` forces every internal type to also be FFI-safe, shrinks the crate to a subset of Rust, and makes it untestable with ordinary `cargo test`. Keeping the split clean costs a little duplication at the boundary but pays it back many times over the first time either side needs to change independently.

## Do
```rust
// foo/src/lib.rs — idiomatic, safe, unit-testable
pub struct Message {
    destination: [u8; 8],
    data: Vec<u8>,
}

impl Message {
    pub fn new(destination: [u8; 8], data: Vec<u8>) -> Self {
        Self { destination, data }
    }

    pub fn transmit(&self) -> Result<(), TransmitError> {
        // real send logic lives here, fully testable
        Ok(())
    }
}

pub struct TransmitError;
```

```rust
// foo-ffi/src/lib.rs — pure translation, no logic of its own
use foo::Message;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn transmit_message(
    destination: *const [u8; 8],
    data: *const u8,
    data_len: usize,
) -> u8 {
    let data = unsafe { std::slice::from_raw_parts(data, data_len) }.to_vec();
    let destination = unsafe { *destination };
    match Message::new(destination, data).transmit() {
        Ok(()) => 0,
        Err(_) => 1,
    }
}
```

## Don't
```rust
// foo/src/lib.rs — FFI layout has leaked into the core crate
#[repr(C)]
pub struct Message {
    pub destination: [u8; 8],
    pub data_ptr: *mut u8,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Message {
    pub unsafe fn transmit(&self) -> u8 {
        // business logic now entangled with raw pointer bookkeeping
        0
    }
}
```

## Notes
This is a design-time discipline, not something a linter can catch reliably — review it at PR time by asking whether `foo` could still be unit-tested and consumed by a pure-Rust caller with zero knowledge of the FFI crate.
</content>
