---
id: unsafe-ffi-escape-hatch
title: Give native-handle wrappers unsafe from_native/into_native escape hatches
category: unsafe
severity: info
tier: 1
enforceability: judge
tags: [ffi, interop, handles]
applies_to:
  - "struct wrapping a raw OS/native handle (HANDLE, fd, pointer)"
  - "type that must round-trip through foreign code"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-ESCAPE-HATCHES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a type wraps a native handle, provide `unsafe fn from_native` to construct it from a handle obtained elsewhere, plus `into_native`/`to_native` to hand the raw handle back out, so callers who must cross an FFI boundary are not locked out of your abstraction.

## Why
A safe wrapper that hides its handle completely is only usable through the API you thought of in advance. Interop scenarios are unpredictable: a caller may receive the native handle from a different library, or need to pass your wrapped handle into one. An `unsafe` escape hatch keeps the safe surface intact for normal use while giving advanced callers a documented, deliberate way out — the `unsafe` keyword and the safety-requirements doc comment make clear that upholding the handle's invariants is now their responsibility.

## Do
```rust
type HNative = *const u8;

pub struct Handle(HNative);

impl Handle {
    pub fn new() -> Self {
        // Safely creates the handle via API calls.
        Self(std::ptr::null())
    }

    /// Constructs a `Handle` from a native handle obtained elsewhere.
    ///
    /// # Safety
    ///
    /// `native` must be a valid, currently-open handle of the expected kind,
    /// and must not be used by any other `Handle` at the same time.
    pub unsafe fn from_native(native: HNative) -> Self {
        Self(native)
    }

    pub fn into_native(self) -> HNative {
        self.0
    }

    pub fn to_native(&self) -> HNative {
        self.0
    }
}
```

## Don't
```rust
type HNative = *const u8;

// No way in or out for a caller who already has a native handle,
// or who must hand this type to a foreign API expecting the raw type.
pub struct Handle(HNative);

impl Handle {
    pub fn new() -> Self {
        Self(std::ptr::null())
    }
}
```

## Notes
Document every safety requirement the caller must uphold on `from_native` — what makes the handle valid, who owns it afterward, and any exclusivity constraints. `into_native`/`to_native` are safe because handing out a value you already validated on construction cannot itself violate memory safety; the risk moves to whatever the caller does with it next.
