---
id: obs-no-sensitive-data
title: Never log secrets or PII; redact or skip them
category: observability
severity: warn
tier: 1
enforceability: judge
tags: [tracing, security, pii, redaction]
applies_to:
  - "#[instrument] on a function taking credentials or tokens"
  - "logging a struct that contains a password, key, or PII"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#obs-no-sensitive-data
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep secrets and PII out of logs and spans by skipping sensitive arguments and wrapping sensitive values in redacting types.

## Why
Logs and traces are routinely shipped to cloud aggregators and retained for weeks or months. A password, token, session cookie, or piece of PII in a log line or span field leaks into systems with weaker access controls than your secrets manager, appears in support exports, and can violate GDPR/HIPAA/PCI-DSS. `#[instrument(skip(...))]` and redacting newtypes prevent the leak cheaply.

## Do
```rust
use tracing::{info, instrument};

#[derive(Clone)]
struct Secret(String);

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}

struct Credentials {
    username: String,
    password: Secret,
    api_key: Secret,
}

#[instrument(skip(credentials), fields(username = %credentials.username))]
async fn authenticate(credentials: &Credentials) -> bool {
    info!("authenticating user");
    verify_password(&credentials.username, &credentials.password)
}

fn verify_password(_username: &str, _password: &Secret) -> bool { true }
```

## Don't
```rust
use tracing::instrument;

struct Credentials {
    username: String,
    password: String,
    api_key: String,
}

#[instrument]
async fn authenticate(credentials: &Credentials) -> bool {
    tracing::info!(?credentials, "authenticating user");
    true
}
```

## Notes
Use `skip_all` then add only safe fields with `fields(...)`. The `secrecy` crate provides `Secret<T>` with a `[redacted]` `Debug` impl and an explicit `.expose_secret()` so you know exactly where a value is revealed. Never log full request bodies in production — log only metadata (size, content-type, path).
