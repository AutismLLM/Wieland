---
id: unsafe-no-mangle-unsafe
title: Write #[unsafe(no_mangle)] and friends, not the bare attribute forms in 2024
category: unsafe
severity: warn
tier: 2
enforceability: check
tags: [ffi, edition-2024, linker]
applies_to:
  - "#[no_mangle]"
  - "#[export_name = ...]"
  - "#[link_section = ...]"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['#\[no_mangle\]', '#\[export_name', '#\[link_section']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-no-mangle-unsafe
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In Rust 2024, write `#[unsafe(no_mangle)]`, `#[unsafe(export_name = "...")]`, and `#[unsafe(link_section = "...")]` — not the bare attribute forms.

## Why
These attributes were reclassified as unsafe in Rust 2024 because they can cause undefined behavior with no `unsafe` block at any call site. If two items in the same binary export the same symbol name, the linker silently picks one and discards the other — the winner may have a completely different type or signature, producing type-level UB with no diagnostic. Requiring `#[unsafe(...)]` makes this footgun visible and auditable; the bare forms are a hard error in the 2024 edition.

## Do
```rust
#[unsafe(no_mangle)]
pub extern "C" fn init() {}

#[unsafe(export_name = "plugin_entry")]
pub fn plugin_main() {}

#[unsafe(link_section = ".init_array")]
static INIT: extern "C" fn() = init;
```

## Don't
```rust
#[no_mangle]
pub extern "C" fn init() {}

#[export_name = "plugin_entry"]
pub fn plugin_main() {}

#[link_section = ".init_array"]
static INIT: extern "C" fn() = init;
```

## Notes
The `unsafe(...)` wrapper does not require an `unsafe {}` block at the call site; it marks the attribute itself as load-bearing for safety. Run `cargo fix --edition` to rewrite bare forms automatically, then confirm each exported symbol name is unique across the binary — collisions are especially dangerous in plugin architectures, `cdylib` crates, and embedded firmware with custom linker scripts.
