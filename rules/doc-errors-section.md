---
id: doc-errors-section
title: Give fallible public functions a `# Errors` doc section
category: docs
severity: warn
tier: 2
enforceability: gate
tags: [rustdoc, errors, result]
applies_to:
  - "pub fn returning Result"
rust_edition: "2024"
msrv: null
detect:
  clippy: [missing_errors_doc]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-errors-section
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Document a `# Errors` section on every public function that returns `Result`, stating when and why it fails.

## Why
A `Result` signature tells users a function can fail but not how. The `# Errors` section names the specific conditions and variants, letting callers handle failures without reading the implementation — which they often cannot see in a published library. `clippy::missing_errors_doc` flags public fallible functions that omit it.

## Do
```rust
/// Opens a file and reads its contents as a UTF-8 string.
///
/// # Errors
///
/// Returns an error if:
/// - The file does not exist ([`Error::NotFound`])
/// - The process lacks permission to read it ([`Error::PermissionDenied`])
/// - The file contains invalid UTF-8 ([`Error::InvalidUtf8`])
pub fn read_file(path: &Path) -> Result<String, Error> {
    todo!()
}
```

## Don't
```rust
/// Opens a file and reads its contents.
pub fn read_file(path: &Path) -> Result<String, Error> {
    todo!()
}
```

## Notes
Use intra-doc links to connect each documented condition to its error variant, e.g. `[`ValidationError::TooShort`]`.
