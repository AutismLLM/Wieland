---
id: ctx-no-stored-context
title: Do not store a context.Context in a struct
category: context
language: go
go_version: "1.26"
severity: warn
tier: 2
enforceability: check
tags: [context, api]
applies_to:
  - "struct field ctx context.Context"
detect:
  golangci: [containedctx]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Pass `context.Context` as the first parameter of each call that needs it; do not keep it in a struct field.

## Why
A stored context outlives the request it belonged to, so later calls run under a cancelled or wrong deadline, and the dependency is hidden from the method signatures.

## Do
```go
import (
	"context"
	"database/sql"
)

type Store struct {
	db *sql.DB
}

func (s *Store) Count(ctx context.Context) (int, error) {
	var n int
	err := s.db.QueryRowContext(ctx, "SELECT count(*) FROM items").Scan(&n)
	return n, err
}
```

## Don't
```go
type Store struct {
	ctx context.Context
	db  *sql.DB
}

func (s *Store) Count() (int, error) {
	var n int
	err := s.db.QueryRowContext(s.ctx, "SELECT count(*) FROM items").Scan(&n)
	return n, err
}
```
