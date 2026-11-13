---
id: ctx-request-with-context
title: Build outgoing requests with a context
category: context
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [context, http, cancellation]
applies_to:
  - "http.Get"
  - "http.NewRequest"
  - "net.Dial"
  - "exec.Command"
detect:
  golangci: [noctx]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Use the context-taking form of calls that do I/O: `http.NewRequestWithContext`, `(&net.Dialer{}).DialContext`, `exec.CommandContext`, `db.QueryContext`.

## Why
Without a context the call cannot be cancelled: a slow upstream holds the goroutine after the client gave up, and shutdown waits for it. `http.Get` and friends also have no timeout by default.

## Do
```go
import (
	"context"
	"fmt"
	"net/http"
)

func ping(ctx context.Context, url string) error {
	req, err := http.NewRequestWithContext(ctx, http.MethodHead, url, nil)
	if err != nil {
		return fmt.Errorf("build request: %w", err)
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return fmt.Errorf("head %s: %w", url, err)
	}
	return resp.Body.Close()
}
```

## Don't
```go
func ping(url string) error {
	resp, err := http.Head(url)
	if err != nil {
		return err
	}
	return resp.Body.Close()
}
```
