---
id: type-accept-interfaces-return-structs
title: Accept small interfaces, return concrete types
category: types
language: go
go_version: "1.26"
severity: warn
tier: 0
enforceability: judge
tags: [types, api, interfaces]
applies_to:
  - "func New() Interface"
  - "interface defined next to its only implementation"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Take the smallest interface a function needs (often from the standard library, like `io.Reader`), define interfaces where they are consumed, and return concrete types from constructors.

## Why
A small consumer-side interface makes a function easy to test and reuse; returning an interface hides the concrete type's methods and forces a cast. Models copy the Java habit of an interface per type next to its only implementation.

## Do
```go
import (
	"bufio"
	"io"
)

type LineCounter struct {
	lines int
}

func NewLineCounter() *LineCounter {
	return &LineCounter{}
}

func (c *LineCounter) Count(r io.Reader) (int, error) {
	s := bufio.NewScanner(r)
	for s.Scan() {
		c.lines++
	}
	return c.lines, s.Err()
}
```

## Don't
```go
type LineCounter interface {
	Count(r *os.File) (int, error)
}

type lineCounter struct{ lines int }

func NewLineCounter() LineCounter {
	return &lineCounter{}
}
```
