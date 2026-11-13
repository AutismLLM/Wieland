---
id: err-no-nil-nil
title: Do not return a nil value together with a nil error
category: errors
language: go
go_version: "1.26"
severity: warn
tier: 2
enforceability: check
tags: [errors, api]
applies_to:
  - "return nil, nil"
detect:
  golangci: [nilnil]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
A function returning `(*T, error)` returns either a usable value or a non-nil error; for "not found", return a sentinel error or a `(T, bool)` pair.

## Why
`nil, nil` makes every caller check two things and most check only the error, then dereference nil. A sentinel or a boolean puts the "absent" case in the signature.

## Do
```go
import "errors"

type User struct{ Name string }

var ErrNoUser = errors.New("no such user")

func find(users map[string]*User, id string) (*User, error) {
	u, ok := users[id]
	if !ok {
		return nil, ErrNoUser
	}
	return u, nil
}
```

## Don't
```go
func find(users map[string]*User, id string) (*User, error) {
	u, ok := users[id]
	if !ok {
		return nil, nil
	}
	return u, nil
}
```
