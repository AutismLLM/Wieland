---
id: res-close-sql-rows
title: Close sql.Rows and check rows.Err after the loop
category: resources
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [resources, database]
applies_to:
  - "db.Query"
  - "rows.Next()"
  - "db.Prepare"
detect:
  golangci: [sqlclosecheck, rowserrcheck]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
`defer rows.Close()` right after a successful `Query`, and check `rows.Err()` after the `for rows.Next()` loop; close prepared statements the same way.

## Why
Open rows pin a connection until they are closed, so a leak exhausts the pool. `rows.Next()` returns `false` both at the end and on a failure in the middle; without `rows.Err()` a broken result set looks like a short one.

## Do
```go
import (
	"context"
	"database/sql"
	"fmt"
)

func names(ctx context.Context, db *sql.DB) ([]string, error) {
	rows, err := db.QueryContext(ctx, "SELECT name FROM users")
	if err != nil {
		return nil, fmt.Errorf("query users: %w", err)
	}
	defer rows.Close()
	var out []string
	for rows.Next() {
		var name string
		if err := rows.Scan(&name); err != nil {
			return nil, fmt.Errorf("scan user: %w", err)
		}
		out = append(out, name)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterate users: %w", err)
	}
	return out, nil
}
```

## Don't
```go
func names(ctx context.Context, db *sql.DB) ([]string, error) {
	rows, err := db.QueryContext(ctx, "SELECT name FROM users")
	if err != nil {
		return nil, err
	}
	var out []string
	for rows.Next() {
		var name string
		_ = rows.Scan(&name)
		out = append(out, name)
	}
	return out, nil
}
```
