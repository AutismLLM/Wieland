# Wieland

**Coding rules for code written by LLMs, with a gate that enforces them.**

Wieland keeps one rule corpus per language (Rust, TypeScript, Go) as plain Markdown files and
compiles it into what coding agents read: a Claude Code skill, a compact always-on core, and a
linter configuration. The same corpus drives a gate that runs after every edit an agent makes, in
Claude Code, Codex, Cursor, Gemini CLI or Pi, and before every commit.

The idea: do not prompt what a machine can check. Each rule says how far a machine can take it:

| Class | Meaning | Delivered as |
|---|---|---|
| `gate` | a reliable detector exists | blocks the edit or commit |
| `check` | detectable but noisy | printed as advice |
| `judge` | design taste, no detector | prompt only (skill and core) |
| `compiler` | the compiler already rejects it | usually not a rule |

So the prompt stays small, the gate stays strict, and the full corpus is there to look things up.

## What is in it

| Pack | Rules | Detectors (pinned) | Manifest |
|---|---|---|---|
| Rust | 311 | `cargo clippy` | `Cargo.toml` |
| TypeScript 7 | 20 | `tsc` with required strict flags, `oxlint` type-aware | `tsconfig.json` |
| Go 1.26 | 20 | `golangci-lint` v2.15.0 | `go.mod` |

The Rust pack merges [leonardomso/rust-skills](https://github.com/leonardomso/rust-skills) and
[microsoft/rust-guidelines](https://github.com/microsoft/rust-guidelines) (both MIT, sources in
`vendor/`). The TypeScript and Go packs are original. Two version skills,
`typescript-7-patch-notes` and `go-1-26-patch-notes`, cover what changed since most models were
trained.

## Quick start

Requirements: Rust (stable), and for the other packs Node 22+ with npm, and Go 1.26.

```sh
git clone https://github.com/Beemindapp/Wieland
cd Wieland
bash ci/install.sh        # wieland on PATH, corpus in ~/.wieland, pinned tools, Claude skills
bash ci/check.sh          # the full test gate
```

Gate a project by hand:

```sh
cd your-project
wieland gate --all        # picks the language from the nearest Cargo.toml / tsconfig.json / go.mod
wieland gate --staged     # staged files only, grouped per project (works in monorepos)
```

Hook it into your agent: see [`hooks/README.md`](hooks/README.md) for Claude Code, Codex, Cursor,
Gemini CLI, Pi and a git `pre-commit` hook.

A blocked edit looks like this:

```
src/bad.ts:34: [async-no-floating-promise] Await, return or explicitly void every promise
    fix: Every promise you create is awaited, returned, or handed to a handler that deals with rejection; a deliberate fire-and-forget is written as `void promise.catch(handler)`.
gate: 1 violation(s)
```

## Commands

```
wieland validate [dir]            schema-check a pack (rules, rules/typescript, rules/go)
wieland compile --all [--check]   build dist/ for every pack; --check fails if dist/ is stale
wieland gate [--all|--staged|--changed|--file <paths>]
wieland hook [--agent claude|codex|cursor|gemini|pi] [--path <file>]
wieland check-detectors           every detector named in the corpus exists in the pinned tools
wieland check-examples [--pack rust|typescript|go]   every Do example compiles
wieland install                   copy the corpus to ~/.wieland/rules
```

`wieland-mcp` serves the Rust corpus over MCP (stdio) and plain HTTP.

## Writing a rule

A rule is one Markdown file with YAML frontmatter (`spec/schema/rule.schema.yaml`) and four
sections: `## Rule`, `## Why`, `## Do`, `## Don't`. A `gate` rule must name a real detector, and
every `Do` example must compile under the pack's settings; CI checks both. See
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## Layout

```
rules/            Rust pack; rules/typescript/ and rules/go/ are the other packs
spec/             schema, taxonomies, enforcement and hook specs
crates/           wieland-core (model, validate, compile), wieland-cli, wieland-mcp
tools/typescript/ exact pins of tsc, oxlint, oxlint-tsgolint
skills/           version patch-notes skills
hooks/            agent and git hook templates
ci/               check.sh, check-examples.sh, install.sh, gate fixtures
```
