# Wieland

**The smith-god for Rust code.** One rule corpus, forged into every harness.

Wieland merges the best existing Rust-for-LLM rule sets into a single canonical corpus, then compiles that one source into whatever a coding agent actually consumes — a Claude skill, Cursor `.mdc` rules, `AGENTS.md`, Copilot instructions, an MCP server, an HTTPS endpoint, or a git hook. Write a rule once; every harness gets it.

> Status: **framework scaffold** (2026-07-12). Schema, taxonomy, and all layer specs are defined. Rules are **not yet merged** — see `MERGE-PLAN.md`.

---

## Why

Today the same Rust knowledge is hand-re-encoded per tool and drifts. Wieland makes the corpus the single source of truth and every artifact a deterministic projection of it. The payoff is one auditable rulebook delivered at four pressures — **prompt → retrieval → machine-check → hard gate** — portable across whatever the next generation of agents runs on.

Read `ARCHITECTURE.md` first. It is the contract everything else fills.

---

## Layout

```
wieland/
├── ARCHITECTURE.md     ← start here: the one idea, the tiers, the data flow
├── MERGE-PLAN.md       ← how the two sources become rules/ (deferred)
├── schema/             ← canonical rule schema + one worked example
├── taxonomy/           ← the 26-category namespace
├── rules/             ← THE CORPUS (empty until merge)
├── export/             ← compiler spec: canonical → each harness format
├── mcp/                ← wieland-forge MCP server (stdio + HTTPS)
├── hooks/              ← Claude Code + generic enforcement gates
├── research/           ← internal HTTPS corpus/research endpoint
└── sources/            ← provenance ledger for the merge
```

Raw inputs are the siblings `../rust-skills/` and `../rust-guidelines/` (vendored, `.git` stripped).

---

## Delivery tiers (the core mechanic)

| Tier | Delivery | Blocks? |
|------|----------|---------|
| L0 core | always in the prompt | no |
| L1 on-demand | retrieved by relevance / skill / MCP | no |
| L2 checked | detector runs against real code (clippy/ast) | no |
| L3 enforced | hook / CI gate on the `deny` set | **yes** |

Same corpus. One `tier` field per rule decides the pressure.

---

## What exists now vs. next

- **Now (this scaffold):** the schema, the category namespace, one example rule, and full concept specs for compile / MCP / hooks / HTTPS research / merge.
- **Next (`MERGE-PLAN.md`):** pour rust-skills + Microsoft guidelines into `rules/`, fill detectors, assign tiers, then build the thin `wieland` CLI + MCP server.

Nothing here runs yet. It is the mold; the merge is the pour.
