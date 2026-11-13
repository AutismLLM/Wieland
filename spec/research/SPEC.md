# Wieland — Internal HTTPS Research Endpoint Spec

Not every consumer speaks MCP, and not every consumer wants a git checkout. The research endpoint serves the corpus and its backing research over plain HTTPS so *anything that can `GET`* — a shell script, a CI step, a homegrown agent, a browser — pulls one versioned source of truth.

Status: concept. Shares a process with the MCP HTTP transport (`../mcp/SPEC.md`); MCP tools and these REST routes are the same server, same in-memory corpus.

---

## Routes

| Route | Returns | Consumer |
|-------|---------|----------|
| `GET /bundle/core.txt` | L0 always-on set, condensed | base-prompt injection, hooks |
| `GET /bundle/all.txt` | full corpus, condensed (MS-style) | thin agents, one-shot context stuffing |
| `GET /rules/{id}.md` | one rule, canonical form | targeted lookup, doc links |
| `GET /rules?category=&tag=&tier=` | rule list (json) matching filters | retrieval without MCP |
| `GET /categories.json` | taxonomy | scoping UI/agents |
| `GET /schema.yaml` | the rule schema | authoring tools, validators |
| `GET /research/{slug}.md` | curated deep-dives (see below) | humans, deep agents |
| `GET /version` | corpus hash + rust_edition + build time | cache-busting, staleness checks |

Everything is read-only, cacheable, and content-addressed via `/version`. Auth = bearer token on the internal deployment; the corpus is not secret but the endpoint is not public.

---

## The research corpus (`/research/*`)

Beyond the terse rules, Wieland carries longer-form research an agent can pull when a rule is not enough:

- the Microsoft `all.txt` condensed guidelines (vendored, re-served);
- edition-migration notes (2021 → 2024) with concrete breakage patterns;
- version-tracking: latest stable, MSRV guidance, "no LTS — editions are the stability contract";
- topic deep-dives (async cancellation safety, unsafe soundness, allocation strategy) that individual rules link to via `applies_to`/`Notes`.

These are Markdown files under `research/`, served verbatim. They are *documentation*, not rules — no frontmatter contract, no compile step.

---

## Why a dumb HTTP layer next to a smart MCP one

MCP is the right interface for an agent mid-task. HTTPS `GET` is the right interface for everything else: a `curl` in a Dockerfile that stuffs `core.txt` into a system prompt, a static-site render of the rulebook, a non-MCP editor plugin, a teammate reading it in a browser. One server, two doors — MCP for tools, REST for text — so no consumer is locked out by protocol. Consistent with the house Cloudflare doctrine: keep the source local/internal, expose a thin perimeter, avoid protocol lock-in.

---

## As-built (M6)

The HTTP layer lives in `crates/wieland-mcp/src/http.rs` (crate `tiny_http`), sharing the **same** `Arc<Corpus>` as the MCP server — one process, one corpus load. Two ways to run:

- `wieland-mcp` — stdio MCP server (default). If `WIELAND_HTTP=<addr>` is set, it *also* spawns the HTTP server on a background thread (truly one process serving both doors).
- `wieland-mcp http [addr]` — HTTP-only (default `127.0.0.1:8787`), for a non-MCP deployment.

Routes (all `GET`; other methods → 405):

| Route | Content-Type | Body |
|-------|--------------|------|
| `/version` | `application/json` | `{server, version, corpus_sha, corpus_tag, rule_count}` |
| `/categories.json` | `application/json` | `[{key, prefix, count}, …]` |
| `/bundle/core.txt` | `text/plain` | the compiled L0 core (rendered live from the corpus via the `bundle` target) |
| `/rules/{id}` | `text/markdown` | the raw canonical rule file; unknown id → 404 |

**TLS:** the process serves plain HTTP; TLS is terminated at a reverse proxy (Cloudflare / nginx), per the house doctrine — the binary carries no cert handling. Corpus is loaded from `WIELAND_RULES` (default `./rules`).

**Release discipline:** `corpus_tag` = `wieland-corpus-<first-12-of-sha>`. A release is cut by tagging the commit whose committed `rules/`+`dist/` produce that hash (`git tag wieland-corpus-<sha12>`); checking the tag out and re-running `wieland compile --all --check` reproduces the exact corpus, so `/version` is a verifiable release identifier, not a live-mutating value.

Speculative emitters (Cursor / Copilot / Aider / Codex) remain **unbuilt** per constraint 8 — a standing note, not code, until a real consumer needs one.
