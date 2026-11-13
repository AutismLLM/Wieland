---
id: opt-cache-friendly
title: Lay out data for cache-efficient sequential access
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, cache, data-layout, hot-path]
applies_to:
  - "hot loops that touch one field of a large struct"
  - "array-of-structs vs struct-of-arrays choices"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-cache-friendly
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Organize hot data so the fields a loop touches are contiguous in memory — prefer struct-of-arrays or hot/cold splitting over a fat array-of-structs when a loop reads only some fields.

## Why
An L3 miss costs ~100+ cycles versus ~4 for an L1 hit, so data layout, not instruction count, often dominates. Loading a 40-byte struct to read 12 bytes wastes most of every cache line; storing each field in its own contiguous `Vec` fills cache lines with data you actually use and lets the prefetcher run ahead. This is a layout judgement — measure with `perf stat -e cache-misses` before restructuring.

## Do
```rust
struct Particles {
    positions_x: Vec<f32>,
    velocities_x: Vec<f32>,
}

fn update_positions(p: &mut Particles, dt: f32) {
    for (px, vx) in p.positions_x.iter_mut().zip(&p.velocities_x) {
        *px += vx * dt;
    }
}
```

## Don't
```rust
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    mass: f32,
    id: u64,
    flags: u8,
}

fn update_positions(particles: &mut [Particle], dt: f32) {
    for p in particles {
        p.position[0] += p.velocity[0] * dt;
    }
}
```

## Notes
Struct-of-arrays hurts readability and cross-field access, so apply it only to measured hot loops. For concurrent counters, pad to a cache line (`#[repr(align(64))]`) to avoid false sharing.
