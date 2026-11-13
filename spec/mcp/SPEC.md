# wieland-forge — MCP Server Spec

The live face of the corpus. Where the `export/` compilers produce *static* artifacts, the MCP server serves the *same* `rules/*.md` dynamically and can run detectors against real code. Any MCP-speaking agent (Claude Code, Cursor, Zed, Cline, Codex…) gets one shared source of truth.

Status: concept. Not built. Transport, tool, and schema contracts below are what the implementation must honor.

---

## Transports

| Transport | Use | Auth |
|-----------|-----|------|
| **stdio** | Local, per-developer. Launched by the harness as a child process. Default. | none (process trust) |
| **streamable HTTP(S)** | Team / shared. One server, many agents, one corpus version. Internal endpoint behind TLS. | bearer token; corpus is read-mostly |

The same binary serves both; transport is a launch flag. The HTTP transport doubles as the plain research API (see `../research/SPEC.md`) — MCP clients hit the tool endpoints, dumb HTTP clients hit the REST routes, same corpus in memory.

---

## Tools

All tools are read-only except none — the server never mutates the caller's code; it returns findings and text.

### `wieland.core()`
Returns the L0 always-on rule set as condensed text (`core.txt`). What a harness injects at session start when it detects Rust. Tiny by contract.

### `wieland.lookup(query?, id?, category?, tags?, limit?)`
Retrieval over the corpus. `id` → exact rule. `query` → relevance match across `title`/`applies_to`/body. `category`/`tags` → filtered lists. Returns full rule bodies (frontmatter + prose). This is L1 delivery for agents that pull instead of preload.

### `wieland.review(code_or_path, edition?)`
The L2 workhorse. Runs the corpus's detectors, but is **honest about what each detector needs**:
- **grep + ast detectors** → work on any snippet, always available. Cheap regex + ast-grep/semgrep structural match.
- **clippy detectors** → require a *compilable crate context*, not a loose snippet. Clippy type-checks; it cannot lint a fragment with no crate. So the clippy path runs **only when given a real crate directory**; on a bare snippet it is skipped and the response says so. Do not promise snippet-level clippy — it would silently under-deliver.

Returns `{ rule_id, severity, span, message, fix_hint, detector }[]`, each pointing at a real rule, plus a `coverage` note stating which detector classes actually ran. Importing clippy also imports a toolchain+workspace dependency into an otherwise zero-dep design — that cost is confined to this one tool and declared, never hidden.

### `wieland.explain(lint)`
Given a clippy lint name (e.g. `clippy::unwrap_used`), returns the Wieland rule(s) that own it, with the `Why`/`Do`/`Don't`. Turns a terse lint into teaching.

### `wieland.categories()`
Returns `taxonomy/categories.yaml` — the map an agent uses to scope a `lookup`.

### `wieland.version()`
Returns the corpus content-hash + git tag + `rust_edition` the corpus targets + build date. Lets a consumer detect skew between the server's corpus and a repo's committed artifacts — the release-not-live model made observable.

---

## Return schema (shared)

```jsonc
// a returned rule
{ "id": "mem-with-capacity", "title": "...", "category": "memory",
  "severity": "warn", "tier": 2, "edition": "2024",
  "body_md": "## Rule ...", "sources": ["leonardomso/rust-skills#..."] }

// a review finding
{ "rule_id": "err-no-unwrap-prod", "severity": "deny",
  "path": "src/x.rs", "span": [42, 42], "message": "...",
  "fix_hint": "propagate with ? or handle the None case", "detector": "clippy" }
```

Findings carry `rule_id` so the caller can `explain` or the hook layer can decide block-vs-warn from `severity`.

---

## Why MCP *and* static export

Static exports win when the harness has no runtime (Copilot inline, a CI text check). MCP wins when the agent can call tools mid-edit: it gets fresh corpus without a recompile, and — critically — `review` runs *real clippy*, which no static prompt can do. Wieland ships both because the next generation of harnesses is split between the two and we refuse to pick.
