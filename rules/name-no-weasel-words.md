---
id: name-no-weasel-words
title: Avoid weasel-word type names like Manager, Service, and Factory
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, types, clarity]
applies_to:
  - "type names ending in Manager, Service, Factory, or Helper"
  - "function parameters accepting a *Builder/*Factory instance"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-WEASEL-WORDS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name a type after what it holds or what it specifically does, not after a generic OO role like `Manager`, `Service`, or `Factory`.

## Why
Every type in a program "manages" or "services" something, so those suffixes carry no information a reader can act on. `Bookings` says what the type holds; `BookingDispatcher` says what it specifically does with it. `Manager` in particular smuggles in an unneeded lifecycle-ownership story that `Drop` already covers, and `Factory` imports an OO pattern whose idiomatic Rust name is `Builder` — or, for repeatable construction, a plain `impl Fn() -> Foo` parameter instead of a `FooFactory` argument.

## Do
```rust
struct Bookings {
    entries: Vec<Booking>,
}

struct BookingDispatcher {
    bookings: Bookings,
}

struct Booking;

fn schedule_job(make_job: impl Fn() -> Job) -> Job {
    make_job()
}

struct Job;
```

## Don't
```rust
struct BookingManager {
    entries: Vec<Booking>,
}

struct BookingService {
    manager: BookingManager,
}

struct Booking;

fn schedule_job(factory: JobFactory) -> Job {
    factory.create()
}

struct JobFactory;
struct Job;

impl JobFactory {
    fn create(&self) -> Job {
        Job
    }
}
```

## Notes
A field or local variable named `booking_service` referring to an external system is fine — the objection is to `BookingService` as a *type* name in your own code, not to the English word appearing anywhere.
