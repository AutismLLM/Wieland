---
id: anti-unwrap-abuse
title: Don't use unwrap or expect for recoverable errors in production
category: anti-patterns
severity: deny
tier: 3
enforceability: gate
tags: [correctness, errors, safety]
applies_to:
  - ".unwrap()"
  - ".expect()"
  - "Option or Result in production code"
rust_edition: "2024"
msrv: null
detect:
  clippy: [unwrap_used, expect_used]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-unwrap-abuse
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Do not reach for `.unwrap()` in production code paths; propagate with `?`, supply a default, or handle both cases explicitly.

## Why
`.unwrap()` panics on `None` or `Err`, crashing the program — in production that means lost data, failed requests, and unhappy users. Its panic message also carries no context, so debugging the crash is harder than it needs to be. Every recoverable failure has a better tool: `?` to propagate, `unwrap_or*` for a default, or `match`/`if let` to handle each case.

## Do
```rust
fn load_config() -> Result<Config, Error> {
    let content = std::fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&content)?)
}

let num: i32 = user_input.parse().unwrap_or(0);
let value = map.get("key").ok_or(Error::MissingKey)?;

match receiver.recv() {
    Ok(msg) => handle(msg),
    Err(_) => break,
}
```

## Don't
```rust
let content = std::fs::read_to_string("config.toml").unwrap();
let num: i32 = user_input.parse().unwrap();
let value = map.get("key").unwrap();
let msg = receiver.recv().unwrap();
```

## Notes
`.unwrap()`/`.expect()` are acceptable in tests, in const/static initialization of known-valid data (`Regex::new(r"^\d+$").unwrap()`), and for provable invariants documented with a proof — prefer `expect("why this cannot fail")` over `unwrap()` so the message explains the invariant. Add `#[allow(clippy::unwrap_used)]` at those justified sites.
