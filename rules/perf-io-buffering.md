---
id: perf-io-buffering
title: Wrap Read/Write in BufReader/BufWriter for many small operations
category: performance
severity: warn
tier: 1
enforceability: judge
tags: [performance, io, syscalls]
applies_to:
  - "File or TcpStream read/written in a loop"
  - "byte-by-byte or line-by-line IO on an unbuffered handle"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-io-buffering
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Wrap a raw `Read`/`Write` handle in `BufReader`/`BufWriter` whenever you do many small reads or writes, and flush the writer explicitly.

## Why
Every unbuffered read or write to a file or socket is a syscall, and line- or byte-at-a-time IO can issue millions per second — the kernel-transition cost dwarfs the data transfer. `BufReader`/`BufWriter` batch those into large buffer-sized syscalls, cutting the count by orders of magnitude. This is one of the highest-impact, lowest-effort IO fixes available.

## Do
```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn count_lines(path: &str) -> io::Result<usize> {
    let reader = BufReader::new(File::open(path)?);
    let mut count = 0usize;
    for line in reader.lines() {
        line?;
        count += 1;
    }
    Ok(count)
}

fn write_records(path: &str, records: &[String]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    for record in records {
        writer.write_all(record.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}
```

## Don't
```rust
use std::fs::File;
use std::io::Write;

fn write_records(path: &str, records: &[String]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for record in records {
        file.write_all(record.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
```

## Notes
Always call `flush()?` before a `BufWriter` drops — its implicit flush-on-drop silently discards any error. Skip the extra layer if the underlying type is already buffered (e.g. `tokio::io::BufWriter` in async code).
