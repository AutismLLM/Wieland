---
id: lint-clippy-nursery-selected
title: Cherry-pick high-value clippy::nursery lints instead of enabling the group
category: lints
severity: info
tier: 2
enforceability: check
tags: [clippy, nursery, configuration, noise]
applies_to:
  - "[lints.clippy] tables enabling nursery wholesale"
  - "#![warn(clippy::nursery)]"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['nursery\s*=\s*"(warn|deny)"', 'clippy::nursery']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-clippy-nursery-selected
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Do not enable the whole `clippy::nursery` group; opt into the specific nursery lints that carry their weight, such as `significant_drop_tightening`, `redundant_clone`, and `use_self`.

## Why
Nursery lints are correct in intent but still being refined, so their suggestions can be noisy or wrong at the edges. Turning on the entire group floods you with false positives and creates churn every time a nursery lint changes or graduates. Cherry-picking gives you the genuinely valuable signal — locks held too long, avoidable clones, type names that should be `Self` — without the noise.

## Do
```toml
[lints.clippy]
significant_drop_tightening = "warn"
redundant_clone = "warn"
use_self = "warn"
cognitive_complexity = "warn"
option_if_let_else = "warn"
```

## Don't
```toml
[lints.clippy]
nursery = "warn"
```

## Notes
Start with the small set above and add more nursery lints only after reviewing what they flag against your own codebase.
