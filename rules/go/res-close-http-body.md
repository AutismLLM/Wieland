---
id: res-close-http-body
title: Close every HTTP response body
category: resources
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [resources, http]
applies_to:
  - "http.Get"
  - "client.Do"
  - "resp.Body"
detect:
  golangci: [bodyclose]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
After a successful HTTP request, `defer resp.Body.Close()` before anything else touches the response.

## Why
An unclosed body keeps the connection out of the pool and leaks a goroutine and file descriptor per request. Under load the process runs out of sockets; in tests nothing looks wrong.

## Do
```go
import (
	"context"
	"fmt"
	"io"
	"net/http"
)

func fetch(ctx context.Context, url string) ([]byte, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, url, nil)
	if err != nil {
		return nil, fmt.Errorf("build request: %w", err)
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("get %s: %w", url, err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("get %s: status %d", url, resp.StatusCode)
	}
	return io.ReadAll(resp.Body)
}
```

## Don't
```go
func status(ctx context.Context, url string) (int, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, url, nil)
	if err != nil {
		return 0, err
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return 0, err
	}
	return resp.StatusCode, nil
}
```

## Notes
Closing a body you only read cannot lose data, so the compiled golangci-lint config excludes `(io.ReadCloser).Close` from errcheck and `defer resp.Body.Close()` passes. `Close` on a file you wrote is still checked.
