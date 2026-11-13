---
id: unsafe-implies-ub
title: Mark unsafe only when misuse risks undefined behavior, not merely danger
category: unsafe
severity: warn
tier: 1
enforceability: judge
tags: [safety, api-design, soundness]
applies_to:
  - "unsafe fn signature choice"
  - "unsafe trait declarations"
  - "functions that are dangerous but memory-safe"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-UNSAFE-IMPLIES-UB
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Only mark a function or trait `unsafe` when calling it incorrectly can cause undefined behavior. Do not use `unsafe` as a generic warning label for operations that are merely dangerous, destructive, or expensive but stay memory- and type-safe.

## Why
`unsafe` is a specific contract in Rust: it tells the compiler and the reader that the function has preconditions whose violation is UB, and that the caller — not the compiler — is responsible for upholding them. Slapping `unsafe` on a safe-but-scary function (deleting a database, sending money, formatting a disk) dilutes that signal. Readers start treating `unsafe` blocks as "review carefully" theater instead of "this precondition, if broken, is UB," which is exactly the alarm fatigue that lets a real memory-safety bug slip through review.

## Do
```rust
/// Dereferences `x`.
///
/// # Safety
///
/// `x` must be non-null and point to a valid, initialized `String`.
unsafe fn print_string(x: *const String) {
    // SAFETY: caller guarantees x is valid and initialized.
    println!("{}", unsafe { &*x });
}

fn delete_database(confirm: DeleteConfirmation) -> Result<(), DbError> {
    confirm.execute()
}
```

## Don't
```rust
// Nothing here can produce UB — it's just destructive. `unsafe` is the
// wrong tool; use a type like `DeleteConfirmation` or a `Result` instead.
unsafe fn delete_database() {
    std::fs::remove_dir_all("/var/lib/db").unwrap();
}
```

## Notes
For operations that are dangerous without being UB-risky, communicate the danger through the type system instead — a confirmation type that must be constructed deliberately, a `Result` with a specific error variant, or a name like `force_delete_all` that reads as a warning on its own. Reserve `unsafe` for the narrow, technical meaning the compiler already gives it.
