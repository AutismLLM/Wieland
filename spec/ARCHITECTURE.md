# Wieland — Architecture

> The smith-god for Rust code. One rule corpus, forged into every harness.

Status: **framework scaffold** (2026-07-12). Rules not yet merged. This document defines the shape everything else fills.

---

## 1. The one idea

Every Rust coding-assistant artifact in existence today — Claude skills, Cursor `.mdc` rules, `AGENTS.md`, Copilot instructions, Aider conventions, a giant `all.txt` — is the *same knowledge* re-encoded by hand for a different consumer. They drift. They rot independently. Nobody owns the union.

Wieland inverts it:

```
                      ┌─────────────────────────┐
                      │   rules/*.md            │   ← single source of truth
                      │   (canonical schema)    │      (structured markdown)
                      └────────────┬────────────┘
                                   │  wieland compile <target>
        ┌──────────────┬───────────┼───────────┬──────────────┐
        ▼              ▼           ▼           ▼              ▼
   Claude SKILL   Cursor .mdc   AGENTS.md   Copilot .md    all.txt
   Aider CONV.    MCP corpus    HTTPS API   hook gate      CI check
```

The corpus is dumb, plain, and diffable. Everything downstream is a **deterministic projection** of it — no LLM in the compile path. Add a rule once; every harness gets it on next compile.

That is the whole framework. The rest of this document is the contract that makes the projection possible.

---

## 2. Design principles

1. **Corpus is markdown + frontmatter.** No database, no build step to *read* a rule. A human or any agent can open one file and understand it. (KISS: the center must survive with zero tooling.)
2. **Metadata is machine-truth.** Frontmatter carries everything a compiler, an MCP tool, or a hook needs: id, category, severity, tier, detectors, provenance. Prose is for humans; frontmatter is for machines.
3. **Layers are opt-in.** L0 prompt rules work with nothing installed. MCP, HTTPS, and hooks are *consumers* that add enforcement — never prerequisites.
4. **Harness-neutral by construction.** No rule references Claude, Cursor, or any one tool. Harness specifics live only in `export/` compilers.
5. **Provenance is preserved.** Every rule records where it came from (`leonardomso/rust-skills`, `microsoft/rust-guidelines`, or `wieland` original). Merges are auditable, not lossy.
6. **Edition- and MSRV-aware.** Rust ships every 6 weeks and has no LTS; the corpus pins `rust_edition` and optional `msrv` so advice ages honestly.

---

## 3. Delivery tiers

A single `tier` field on each rule drives *how aggressively* it reaches the model. Same corpus, four delivery pressures.

| Tier | Name | Delivery | Cost | Example |
|------|------|----------|------|---------|
| **L0** | Core | Always-on. Injected into base instructions of every harness. Kept < ~2k tokens total. | Free (in every prompt) | "No `.unwrap()` in prod paths." |
| **L1** | On-demand | Retrieved by relevance — Claude skill load, Cursor glob match, MCP `lookup`. Full 265+ corpus. | Paid only when relevant | `mem-with-capacity`, `async-no-lock-await` |
| **L2** | Checked | Backed by a machine detector (clippy lint / grep / ast query). MCP `review` runs it against real code. | Paid on review | `err-no-unwrap-prod` → clippy `unwrap_used` |
| **L3** | Enforced | Wired into a hook / CI gate. Violations *block* the edit or commit. | Paid at write/commit time | rustfmt-clean, tier-0 deny set |

A rule's tier is a promise about maturity: L3 rules must have a reliable detector; L0 rules must be universally true. Most rules start L1 and earn their way up.

**Tier is derived, not guessed.** Each rule also declares an `enforceability` — its mechanical ceiling (`gate` / `check` / `judge`). Tier falls out of *enforceability × frequency*: the whole "slim base skill, strong references" strategy is "gate what a machine can catch, prompt only the judgement layer." That logic — including the honest ~50/20/30 gate/check/judge split of the corpus and the four enforcement rings (rustc → rustfmt → clippy → cargo-deny/miri/ast-grep) — lives in `ENFORCEMENT.md`. Read it to understand why the prompt stays small.

---

## 4. Data flow, end to end

**Author time** — a human or agent writes/edits `rules/<id>.md` against `schema/rule.schema.yaml`. CI validates frontmatter.

**Compile time** — `wieland compile claude-code` (spec in `export/COMPILE.md`) walks the corpus, filters by target's tier/severity policy, and emits the native artifact (`SKILL.md` + `rules/`, or `.cursor/rules/*.mdc`, etc.). Pure function: same corpus in, same bytes out.

**Serve time** — the same corpus is exposed live:
- as an **MCP server** (`mcp/SPEC.md`) over stdio (local) and streamable HTTPS (team), tools: `lookup / review / explain / core / categories`;
- as a plain **HTTPS research API** (`research/SPEC.md`) for agents that speak HTTP but not MCP: `GET /rules/{id}`, `/bundle/core.txt`, `/bundle/all.txt`.

**Enforce time** — hooks (`hooks/SPEC.md`) fire on `.rs` writes and commits, running the L2/L3 detectors and blocking on the deny set.

Nothing in this loop requires an LLM except *authoring* a new rule. Compile, serve, and enforce are deterministic.

---

## 5. Directory map

```
wieland/
├── ARCHITECTURE.md          this file — the contract
├── README.md                quickstart, tree, status board
├── MERGE-PLAN.md            how 265 + MS rules become one corpus (the "later" step)
├── schema/
│   ├── rule.schema.yaml     canonical rule frontmatter schema (JSON Schema)
│   └── EXAMPLE.rule.md      one fully-worked rule in canonical form
├── taxonomy/
│   └── categories.yaml      the merged category namespace (26 → normalized)
├── rules/                   THE CORPUS (empty until merge; .gitkeep placeholder)
├── export/
│   └── COMPILE.md           compiler spec + every harness target in one place
├── mcp/
│   └── SPEC.md              wieland-forge MCP server design
├── hooks/
│   └── SPEC.md              Claude Code + generic enforcement hooks
├── research/
│   └── SPEC.md              internal HTTPS research/corpus endpoint
└── sources/
    └── PROVENANCE.md        source→rule mapping, license notes
```

Sibling folders `../rust-skills/` and `../rust-guidelines/` are the vendored raw inputs; Wieland consumes them at merge, never depends on them at runtime.

---

## 6. What "next-generation Rust" means here

The corpus does not just say "write idiomatic Rust." It encodes the *decisions an expert makes and a model skips*: choosing `Cow` over eager clone, `let-else` over nested `if let`, bounded channels over unbounded, `#[non_exhaustive]` on public enums, safety comments on every `unsafe` block, `parse-don't-validate` typing. Each is a small rule with a detector where one exists, so the guidance is not vibes — it is checkable.

Wieland's bet: the win is not a better prompt, it is **the same expert knowledge delivered at four pressures (prompt / retrieval / check / gate) from one auditable source**, portable to whatever harness the next generation of agents runs on.

---

## 7. Non-goals (for now)

- Not a linter. Wieland *orchestrates* clippy/rustfmt/miri; it does not reimplement them.
- Not a package. The core is files. Tooling (`wieland compile`, the MCP server) is a thin optional layer, built at implementation time, not scaffolded here.
- Not merged yet. This scaffold defines the schema and projections; `MERGE-PLAN.md` covers turning the two vendored sources into `rules/`.
