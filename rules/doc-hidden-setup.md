---
id: doc-hidden-setup
title: Hide doc-example setup lines with a `# ` prefix
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, examples, doctest]
applies_to:
  - "# Examples block with imports or fixture setup"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-hidden-setup
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Prefix imports, fixtures, and boilerplate in doc examples with `# ` so they compile in the doc-test but stay hidden from the rendered page.

## Why
Setup code — `use` lines, mock construction, `Ok(())` returns — distracts from the one call the example is teaching. The `# ` prefix keeps that code in the compiled doc-test (so the example still runs) while hiding it from readers, leaving only the relevant lines visible.

## Do
```rust
/// Processes a batch of items.
///
/// # Examples
///
/// ```
/// # use my_crate::{Processor, Config, Item, Error};
/// # let config = Config { batch_size: 100, timeout_ms: 5000, retry_count: 3 };
/// # let processor = Processor::new(config);
/// # let items = vec![Item::new("a"), Item::new("b")];
/// let results = processor.process_batch(&items)?;
/// assert!(results.all_succeeded());
/// # Ok::<(), Error>(())
/// ```
pub fn process_batch(&self, items: &[Item]) -> Result<Results, Error> {
    todo!()
}
```

## Don't
```rust
/// Processes a batch of items.
///
/// # Examples
///
/// ```
/// use my_crate::{Processor, Config, Item};
/// let config = Config { batch_size: 100, timeout_ms: 5000, retry_count: 3 };
/// let processor = Processor::new(config);
/// let items = vec![Item::new("a"), Item::new("b")];
/// let results = processor.process_batch(&items)?;
/// assert!(results.all_succeeded());
/// # Ok::<(), my_crate::Error>(())
/// ```
pub fn process_batch(&self, items: &[Item]) -> Result<Results, Error> {
    todo!()
}
```

## Notes
Do not hide setup when the setup *is* the point — for example, a builder-configuration example should show the builder calls in full.
