---
id: conc-vet-clean
title: "Keep go vet clean: no copied locks, lost cancels or bad format verbs"
category: concurrency
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [concurrency, correctness]
applies_to:
  - "sync.Mutex passed by value"
  - "context.WithCancel without cancel"
  - "Printf verbs"
detect:
  golangci: [govet]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Code passes `go vet`: never copy a value that contains a `sync.Mutex` or `sync.WaitGroup`, always call the `cancel` from `context.WithCancel`/`WithTimeout`, and match `Printf` verbs to their arguments.

## Why
These are bugs the compiler accepts. A copied mutex guards nothing; a dropped `cancel` leaks the context's timer and goroutine; a wrong verb prints garbage into logs and errors. `go vet` finds them exactly, so there is no reason to ship them.

## Do
```go
import (
	"context"
	"sync"
	"time"
)

type Counter struct {
	mu sync.Mutex
	n  int
}

func (c *Counter) Inc() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.n++
}

func withDeadline(parent context.Context, work func(context.Context) error) error {
	ctx, cancel := context.WithTimeout(parent, 5*time.Second)
	defer cancel()
	return work(ctx)
}
```

## Don't
```go
type Counter struct {
	mu sync.Mutex
	n  int
}

func (c Counter) Inc() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.n++
}

func withDeadline(parent context.Context, work func(context.Context) error) error {
	ctx, _ := context.WithTimeout(parent, 5*time.Second)
	return work(ctx)
}
```
