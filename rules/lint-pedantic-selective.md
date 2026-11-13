---
id: lint-pedantic-selective
title: Enable clippy::pedantic as a baseline and allow the noisy lints back off
category: lints
severity: info
tier: 1
enforceability: judge
tags: [clippy, pedantic, configuration, taste]
applies_to:
  - "[lints.clippy] configuration"
  - "#![warn(clippy::pedantic)]"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-pedantic-selective
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Treat `clippy::pedantic` as a curated baseline: enable the group, then explicitly `allow` the handful of lints that fight your style instead of enabling it raw or avoiding it entirely.

## Why
Pedantic lints are opinionated and not universally right, so turning the group on unfiltered buries useful suggestions under noise like `missing_errors_doc`, `too_many_lines`, and `module_name_repetitions`. Enabling the group and then allowing the few that do not fit keeps the valuable ones — `doc_markdown`, `semicolon_if_nothing_returned`, `wildcard_imports` — while silencing the arbitrary thresholds. Which lints to keep is a team taste decision, not a mechanical one.

## Do
```toml
[lints.clippy]
pedantic = "warn"

missing_errors_doc = "allow"
missing_panics_doc = "allow"
module_name_repetitions = "allow"
too_many_lines = "allow"
must_use_candidate = "allow"
```

## Don't
```toml
[lints.clippy]
pedantic = "warn"
```

## Notes
If even a curated group is too much churn, opt in to only the specific pedantic lints you want (`doc_markdown`, `semicolon_if_nothing_returned`, `unused_self`, `wildcard_imports`) rather than the group. Agree the keep/allow list as a team and record it so it does not drift.
