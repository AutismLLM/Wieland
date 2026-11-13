---
id: mem-boxed-slice
title: Use Box<[T]> instead of Vec<T> for fixed-size heap data
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, layout, allocation]
applies_to:
  - "Vec field that never grows after construction"
  - "into_boxed_slice"
  - "immutable String field"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-boxed-slice
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a heap collection is fixed at construction and never grows, store it as `Box<[T]>` (or `Box<str>`) instead of `Vec<T>` (or `String`).

## Why
`Vec<T>` carries three words — pointer, length, and capacity — while `Box<[T]>` carries only pointer and length, saving 8 bytes per instance and dropping the unused capacity slack. Across millions of instances that adds up, and the type itself documents the intent: this data is fixed-size. Convert with `into_boxed_slice`, which also shrinks any excess capacity.

## Do
```rust
struct Document {
    paragraphs: Box<[Paragraph]>,
}

fn load_document(data: &[u8]) -> Document {
    let paragraphs: Vec<Paragraph> = parse_paragraphs(data);
    Document {
        paragraphs: paragraphs.into_boxed_slice(),
    }
}
```

## Don't
```rust
struct Document {
    paragraphs: Vec<Paragraph>,
}

fn load_document(data: &[u8]) -> Document {
    let paragraphs: Vec<Paragraph> = parse_paragraphs(data);
    Document { paragraphs }
}
```

## Notes
Keep `Vec<T>` whenever the collection may grow or shrink later. If you need to grow a `Box<[T]>`, `into_vec()` converts back.
