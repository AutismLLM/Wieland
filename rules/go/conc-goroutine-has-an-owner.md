---
id: conc-goroutine-has-an-owner
title: Start a goroutine only with a way to stop it and wait for it
category: concurrency
language: go
go_version: "1.26"
severity: deny
tier: 0
enforceability: judge
tags: [concurrency, lifecycle]
applies_to:
  - "go func()"
  - "background worker"
  - "fire-and-forget goroutine"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Every `go` statement has an owner that can stop the goroutine (a context or a closed channel) and wait for it to finish (`sync.WaitGroup`, `errgroup`, or a done channel).

## Why
A goroutine with no stop signal leaks for the life of the process; one nobody waits for keeps running during shutdown and its errors vanish. Models write `go func(){...}()` as if it were free. No linter sees ownership, so this is judgement.

## Do
```go
import (
	"context"
	"sync"
)

func runWorkers(ctx context.Context, jobs <-chan string, handle func(string)) {
	var wg sync.WaitGroup
	for range 4 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				select {
				case <-ctx.Done():
					return
				case job, ok := <-jobs:
					if !ok {
						return
					}
					handle(job)
				}
			}
		}()
	}
	wg.Wait()
}
```

## Don't
```go
func runWorkers(jobs <-chan string, handle func(string)) {
	for range 4 {
		go func() {
			for job := range jobs {
				handle(job)
			}
		}()
	}
}
```
