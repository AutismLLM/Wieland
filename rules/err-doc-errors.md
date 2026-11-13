---
id: err-doc-errors
title: Document error conditions in a # Errors doc section
category: errors
severity: warn
tier: 2
enforceability: check
tags: [docs, errors, api]
applies_to:
  - "pub fn returning Result"
  - "public API documentation"
rust_edition: "2024"
msrv: null
detect:
  clippy: [missing_errors_doc]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-doc-errors
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give every public fallible function a `# Errors` doc section describing when and why it returns `Err`.

## Why
Callers need to know what can go wrong to handle it correctly. The `# Errors` section is the standard Rust convention for describing failure conditions, and it forms part of the function's contract. Without it a caller has to read the source to learn which inputs fail and which error variants to match. `clippy::missing_errors_doc` enforces the section on public `Result`-returning functions.

## Do
```rust
/// Loads a configuration from the specified path.
///
/// # Errors
///
/// Returns an error if:
/// - the file at `path` does not exist or cannot be read,
/// - the file contents are not valid TOML,
/// - a required configuration key is missing.
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}
```

## Don't
```rust
/// Loads a configuration from the specified path.
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}
```

## Notes
Link to specific error variants with intra-doc links (`[`ConfigError::NotFound`]`) when it helps callers match. Use a bullet list for several conditions, `Returns [...]` statements when each maps to a variant, or prose for complex validation. If the function can also panic, document that separately under `# Panics`.
