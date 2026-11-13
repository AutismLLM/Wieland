---
id: own-move-large
title: Box large types so moves copy a pointer, not the whole value
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [move, box, memcpy]
applies_to:
  - "struct hundreds of bytes or larger moved repeatedly"
  - "enum with one much-larger variant"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-move-large
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a large value is moved repeatedly, store the bulk behind a `Box` so each move copies an 8-byte pointer instead of the entire payload — or borrow it when ownership transfer is not needed.

## Why
A move in Rust is a bytewise copy of the value plus invalidation of the source. For a multi-kilobyte struct that memcpy runs on every pass-by-value and every return. Boxing the heavy field drops the moved size to a pointer, making moves cheap regardless of the data behind it. If you never need ownership, a `&`/`&mut` reference avoids the move altogether.

## Do
```rust
struct GameState {
    board: Box<[[Cell; 100]; 100]>,
    history: Vec<Move>,
    players: [Player; 4],
}

fn process_state(mut state: GameState) -> GameState {
    state.apply_rules();
    state
}
```

## Don't
```rust
struct GameState {
    board: [[Cell; 100]; 100],
    history: [Move; 1000],
    players: [Player; 4],
}

fn process_state(state: GameState) -> GameState {
    let mut new_state = state;
    new_state.apply_rules();
    new_state
}
```

## Notes
Profile before boxing — small types (under ~128 bytes) or rarely-moved values gain nothing, and the extra indirection costs a cache miss on access. Check `std::mem::size_of` to confirm the type is actually large.
