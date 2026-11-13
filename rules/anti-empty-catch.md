---
id: anti-empty-catch
title: Never silently discard an error — log it, propagate it, or handle it
category: anti-patterns
severity: warn
tier: 2
enforceability: gate
tags: [error-handling, correctness]
applies_to:
  - "let _ = fallible_call()"
  - "match arm Err(_) => {}"
  - ".ok() that drops the error"
rust_edition: "2024"
msrv: null
detect:
  clippy: [let_underscore_must_use, let_underscore_drop]
  grep: ['let _ = ', '\.ok\(\);']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-empty-catch
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Acknowledge every error — propagate it with `?`, handle it, or at least log it — rather than dropping it with `let _ =`, an empty `Err(_)` arm, or `.ok()`.

## Why
Silently discarding an error makes failures invisible: bugs hide, corrupted state spreads, and debugging becomes guesswork because the one signal that something went wrong was thrown away. Even a non-critical failure deserves a log line so it can be found later.

## Do
```rust
if let Err(e) = write_to_file(data) {
    error!("failed to write file: {e}");
}

match database.save(record) {
    Ok(_) => info!("saved"),
    Err(e) => warn!("save failed: {e}"),
}
```

## Don't
```rust
let _ = write_to_file(data);

match database.save(record) {
    Ok(_) => println!("saved"),
    Err(_) => {}
}
```

## Notes
Deliberately ignoring an error is acceptable when it is truly not actionable — a socket shutdown, a best-effort metric — but make it explicit with a `// INTENTIONAL:` comment explaining why, so the drop reads as a decision rather than an oversight.
