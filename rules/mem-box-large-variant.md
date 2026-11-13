---
id: mem-box-large-variant
title: Box large enum variants to shrink the whole enum
category: memory
severity: warn
tier: 2
enforceability: check
tags: [performance, memory, allocation, cache]
applies_to:
  - "enum with one much larger variant"
  - "recursive enum type"
rust_edition: "2024"
msrv: null
detect:
  clippy: [large_enum_variant]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-box-large-variant
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When one enum variant is far larger than the others, box it so the enum's size collapses to a pointer for that case.

## Why
An enum is sized to its largest variant, so a single big variant makes every instance — even the empty ones — pay for that size. A `Vec<Message>` where one variant holds a 1 KB buffer wastes roughly 1 KB per element regardless of which variant each element actually is. Boxing the large variant moves its payload to the heap, shrinking the enum to its next-largest variant and improving cache density. `clippy::large_enum_variant` flags this.

## Do
```rust
struct ImageData {
    data: [u8; 1024],
    width: u32,
    height: u32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
    Image(Box<ImageData>),
}
```

## Don't
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
    Image {
        data: [u8; 1024],
        width: u32,
        height: u32,
    },
}
```

## Notes
Don't box when variants are similar in size or all small (under ~64 bytes) — the extra indirection and allocation aren't worth it. Recursive types (linked lists, ASTs) *must* box the recursive variant, since an unboxed recursive enum has infinite size and won't compile.
