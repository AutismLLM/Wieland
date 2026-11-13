# Rust "Skills" & Guides for LLM Coding Agents

Curated 2026-07-12. Latest stable Rust at time of writing: **1.97.0** (2026-07-09). Rust has **no LTS** — 6-week stable train; long-term stability comes from **editions** (current: 2024).

## The two that matter

### 1. leonardomso/rust-skills  (Claude Code skill — install this)
- **What:** 265 rules across 26 categories for writing idiomatic, fast, safe Rust. Each rule is small/focused so the agent pulls in only what's relevant.
- **Format:** Native **Agent Skill** — works with Claude Code, Cursor, Windsurf, Copilot, Codex, Aider, Zed, Amp, Cline.
- **Install (Claude Code):**
  - CLI: `npx add-skill leonardomso/rust-skills`
  - Global manual: `git clone https://github.com/leonardomso/rust-skills.git ~/.claude/skills/rust-skills`
  - Per-project: `git clone https://github.com/leonardomso/rust-skills.git .claude/skills/rust-skills`
- **Use:** `/rust-skills review this function`, `/rust-skills is my error handling idiomatic?`, `/rust-skills is this unsafe block sound?`
- Repo: https://github.com/leonardomso/rust-skills

### 2. microsoft/rust-guidelines — "Pragmatic Rust Guidelines"
- **What:** Design guidelines for idiomatic Rust that scales; builds on the official Rust API Guidelines. Written with LLM consumption in mind.
- **Agent format:** ships a **condensed AI version** to drop into agent/Copilot instructions:
  - `src/agents/all.txt` — full condensed guideline text
  - `src/agents/all.meta` — size/token info
  - Agents & LLMs page: https://microsoft.github.io/rust-guidelines/agents/index.html
- Repo: https://github.com/microsoft/rust-guidelines

## Core idioms these enforce (the recurring themes)
- No lazy `.clone()` to dodge lifetimes; no gratuitous `.unwrap()`; avoid `unsafe`.
- Prefer enums, newtypes, named methods, and self-documenting API shapes.
- Make APIs look like the majority of Rust code — familiarity helps both humans and LLMs.

## Other / adjacent
- Enzo Lombardi write-up: "Implementing Microsoft's Rust Guidelines for AI Coding Agents" — https://medium.com/rustaceans/implementing-microsofts-rust-guidelines-for-ai-coding-agents-bf985dd60fc6
- 0xMassi/claude-skills — 12 Claude Code skills (incl. Rust) enforcing quality/security/modern patterns across TS/Rust/Swift/Go/JS/PG — https://github.com/0xMassi/claude-skills
- Max Woolf, "An AI agent coding skeptic tries AI agent coding" (Rust-heavy, real-world) — https://minimaxir.com/2026/02/ai-agent-coding/

## Recommendation for this repo
For a Rust workspace with a Dioxus client, a gateway and an embedder, install **leonardomso/rust-skills** as a project skill and lift the **Microsoft `all.txt`** condensed rules into a Rust AGENTS/CLAUDE section for review passes. Zero-dep, no runtime cost — pure prompt-side guidance.
