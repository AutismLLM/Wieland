---
id: type-checked-type-assertion
title: Use the two-value form of type assertions
category: types
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [types, safety]
applies_to:
  - "v.(T)"
  - "x.(*Foo)"
detect:
  golangci: [forcetypeassert]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Write `t, ok := v.(T)` and handle `!ok`, or use a type switch; never the one-value `v.(T)`.

## Why
The one-value form panics when the dynamic type differs. The value usually comes from an `any`, a map or a context, exactly the places where the type can change without the compiler noticing.

## Do
```go
import "fmt"

func asString(v any) (string, error) {
	s, ok := v.(string)
	if !ok {
		return "", fmt.Errorf("want string, got %T", v)
	}
	return s, nil
}
```

## Don't
```go
func asString(v any) string {
	return v.(string)
}
```
