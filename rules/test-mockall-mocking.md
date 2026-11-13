---
id: test-mockall-mocking
title: Generate trait mocks with mockall instead of hand-writing them
category: testing
severity: info
tier: 1
enforceability: judge
tags: [mocking, mockall, dependencies, unit-tests]
applies_to:
  - "traits abstracting external dependencies"
  - "tests that need to control and verify collaborator calls"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-mockall-mocking
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `mockall`'s `#[automock]` to generate mock implementations of a dependency trait, then set expectations on the mock to isolate the unit under test.

## Why
Unit tests should not touch real databases, HTTP servers, or the file system — those make tests slow, flaky, and unable to exercise error paths on demand. `mockall` generates a `Mock*` type with configurable return values, argument matchers, and call-count expectations that are verified on drop, giving you full control without a hand-written double.

## Do
```rust
use mockall::automock;

#[automock]
trait Database {
    fn get_user(&self, id: u64) -> Option<User>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn find_user_returns_named_user() {
        let mut mock = MockDatabase::new();
        mock.expect_get_user()
            .with(eq(42))
            .returning(|_| Some(User { id: 42, name: "Alice".into() }));

        let service = UserService::new(mock);

        assert_eq!(service.find_user(42).unwrap().name, "Alice");
    }
}
```

## Don't
```rust
#[test]
fn find_user_returns_named_user() {
    let db = PostgresDatabase::connect("postgres://localhost/test").unwrap();
    let service = UserService::new(db);

    assert_eq!(service.find_user(42).unwrap().name, "Alice");
}
```
