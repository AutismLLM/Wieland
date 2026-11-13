---
id: ctx-first-parameter
title: Make context.Context the first parameter, named ctx
category: context
language: go
go_version: "1.26"
severity: info
tier: 1
enforceability: judge
tags: [context, api]
applies_to:
  - "func f(id string, ctx context.Context)"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
A function that takes a context takes it first and calls it `ctx`; never pass `nil` as a context, use `context.TODO()` while wiring one through.

## Why
The convention lets readers spot cancellable work at a glance and keeps call sites uniform. A nil context panics inside the standard library the first time it is used.

## Do
```go
import (
	"context"
	"time"
)

func wait(ctx context.Context, d time.Duration) error {
	select {
	case <-time.After(d):
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}
```

## Don't
```go
func wait(d time.Duration, c context.Context) error {
	select {
	case <-time.After(d):
		return nil
	case <-c.Done():
		return c.Err()
	}
}
```
