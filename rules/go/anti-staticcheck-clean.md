---
id: anti-staticcheck-clean
title: Keep staticcheck clean
category: anti-patterns
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [correctness, simplicity]
applies_to:
  - "deprecated API"
  - "impossible condition"
  - "unused result"
detect:
  golangci: [staticcheck]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Code passes staticcheck: no deprecated APIs, no conditions that are always true or false, no discarded results of pure functions, no `time.Tick` leaks.

## Why
staticcheck's checks are precise and its findings are almost always real bugs or dead code: `strings.Replace` result ignored, `defer` in a loop, `x == x`, `rand.Seed`. Models reach for deprecated APIs from older training data.

## Do
```go
import (
	"strings"
	"time"
)

func normalise(name string) string {
	return strings.ReplaceAll(strings.TrimSpace(name), " ", "-")
}

func every(d time.Duration, stop <-chan struct{}, tick func()) {
	t := time.NewTicker(d)
	defer t.Stop()
	for {
		select {
		case <-t.C:
			tick()
		case <-stop:
			return
		}
	}
}
```

## Don't
```go
func normalise(name string) string {
	strings.TrimSpace(name)
	return strings.Replace(name, " ", "-", -1)
}

func every(d time.Duration, tick func()) {
	for range time.Tick(d) {
		tick()
	}
}
```
