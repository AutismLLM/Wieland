---
id: test-mock-traits
title: Depend on traits, not concrete types, so dependencies can be mocked
category: testing
severity: info
tier: 1
enforceability: judge
tags: [testability, traits, dependency-injection]
applies_to:
  - "structs holding a concrete database, HTTP, or filesystem client"
  - "code that is hard to unit-test without real infrastructure"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-mock-traits
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Express external dependencies as traits and have services depend on the trait (generic or `dyn`), so tests can inject a fake, stub, or mock in place of the real backend.

## Why
A struct that stores a concrete `PostgresConnection` can only be tested against a real Postgres — slow, flaky, and unable to simulate timeouts or error branches. Putting the dependency behind a trait lets production supply the real implementation while tests supply a lightweight in-memory double, so edge cases and failure paths become fast, deterministic unit tests.

## Do
```rust
#[async_trait]
trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>, DbError>;
}

struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    async fn get_user(&self, id: u64) -> Result<User, Error> {
        self.repo.find_by_id(id).await?.ok_or(Error::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockUserRepo {
        users: std::collections::HashMap<u64, User>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn find_by_id(&self, id: u64) -> Result<Option<User>, DbError> {
            Ok(self.users.get(&id).cloned())
        }
    }

    #[tokio::test]
    async fn get_user_missing_id_is_not_found() {
        let service = UserService { repo: MockUserRepo { users: Default::default() } };
        assert!(matches!(service.get_user(999).await, Err(Error::NotFound)));
    }
}
```

## Don't
```rust
struct UserService {
    db: PostgresConnection,
}

impl UserService {
    async fn get_user(&self, id: u64) -> Result<User, Error> {
        self.db
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .await
    }
}
```
