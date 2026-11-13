# Wieland — Compiler & Export Targets

`wieland compile <target>` is a **pure function**: `rules/*.md` in, one harness-native artifact out. No LLM, no network. Deterministic bytes so diffs are meaningful and CI can assert "corpus changed but exports stale".

Implementation is deferred (this is a scaffold). This file is the spec every target compiler must satisfy.

---

## Compile pipeline (shared by all targets)

1. **Load** every `rules/*.md`, parse frontmatter, validate against `schema/rule.schema.yaml`. Reject the whole build on any invalid rule.
2. **Filter** by the target's tier/severity policy (table below).
3. **Order** by `category` (taxonomy order) then `id`.
4. **Render** each rule through the target's template.
5. **Emit** to the target path. Idempotent: re-running with an unchanged corpus rewrites identical bytes.

A rule's body (`## Rule / Why / Do / Don't / Notes`) renders differently per target: some want the full body, condensed bundles want only `## Rule` + a trimmed `Why`.

---

## Targets

| Target | Output path | Include policy | Body | Notes |
|--------|-------------|----------------|------|-------|
| `claude-code` | `SKILL.md` + `rules/<id>.md` | all tiers; L0 inlined into SKILL.md, L1+ as loadable files | full | Progressive disclosure: SKILL.md is the index, `/wieland <query>` loads matching rule files. |
| `cursor` | `.cursor/rules/<category>.mdc` | tier 0–2 | full | One `.mdc` per category with `globs: ["**/*.rs"]` frontmatter; L0 gets `alwaysApply: true`. |
| `codex` | `AGENTS.md` | tier 0–1 | condensed | Single file, sectioned by category. Codex reads it from repo root. |
| `copilot` | `.github/copilot-instructions.md` | tier 0 only + top-N tier 1 | condensed | Copilot budget is tight; ship the core plus highest-severity L1. Log what was dropped. |
| `aider` | `CONVENTIONS.md` | tier 0–1 | condensed | Aider `--read CONVENTIONS.md` or `read:` in `.aider.conf.yml`. |
| `bundle` | `dist/all.txt` + `dist/core.txt` | all / L0 | condensed | MS-style flat text. `core.txt` is the L0 always-on set (< ~2k tokens). Feeds the HTTPS API and any harness not listed above. |

### Tier/severity policy notes
- **L0 core** appears in *every* target — it is the non-negotiable floor. `bundle` emits it separately as `core.txt` so other layers (hooks, HTTPS, base prompts) can inject just the floor cheaply.
- **L0 has a hard token budget.** `wieland compile` **fails** if the compiled `core.txt` exceeds ~2k tokens. 265 rules with per-rule tiers will otherwise inflate L0 with no backpressure; the failing build forces an explicit demote-something decision on every addition. The budget is enforced by the compiler, not by discipline.
- **Copilot / condensed targets** must call `log()`-equivalent when they truncate, so a thin export never silently reads as full coverage.
- **`deny` severity** rules are the tier-0 promotion pool and the hook/CI deny set (see `../hooks/SPEC.md`).
- **The L3 deny set compiles to a real `clippy.toml` + `[lints]` table**, not prose. `wieland compile deny-set` emits machine config a plain `cargo clippy` consumes, so enforcement never depends on an agent reading English.

---

## Invariants a compiler must uphold

- **Stable ids in, stable anchors out.** A rule id maps to the same heading/anchor/filename across rebuilds so external links don't rot.
- **No target invents content.** If a rule lacks a `## Do` block, the renderer omits the section — it never fabricates an example.
- **Round-trippable provenance.** Every emitted rule keeps a machine-readable `id`, so `wieland review` output can point back to the exact source rule.
- **Fail closed.** One schema-invalid rule fails the entire compile; partial corpora never ship.

---

## CI shape (spec)

The drift gate is the **first** thing to build — before any second export target. Staleness is this design's one real failure mode: the moment a committed artifact diverges from the corpus, the "diffable, reviewable" promise rots silently across every format. Do not add emitters until this gate exists.

```
wieland validate                 # schema-check every rule; fail on any invalid/dup id
wieland compile --all --check    # recompile every target; FAIL on any diff vs committed
```

Additionally, every generated artifact carries a **corpus content-hash header** (`# wieland-corpus: <sha>`) so a human spots drift by eye without running CI, and the MCP server / bundle report the same hash as their `corpus version`. Corpus updates are treated as **releases** (git tag the hash), not live pushes — rules change on human-review timescales, so "instantly live" buys nothing here.

---

## As-built (M2)

The compiler is implemented in `crates/wieland-core/src/compile/`. Only **two** targets are built — `bundle` and `claude-code` — per constraint 8 (Cursor/Copilot/Aider/Codex stay speculative until a real consumer needs one). The rows for those in the table above are the standing spec, not shipped code.

- **Pipeline:** `Corpus::load` → `ordered()` (sort by `category` then `id`) / `tier0()` (tier-0 only, sorted by `id`) → per-target `render()` → `write_artifacts()` or `check_drift()`. Rendering is pure; only the CLI touches the filesystem.
- **Corpus SHA:** SHA-256 (crate `sha2`) over each rule's `id` + full raw file bytes (NUL-separated), rules sorted by id. Hex-encoded. Any frontmatter or body change moves the hash, so every artifact's header flips — that is what makes the drift gate catch an uncompiled edit.
- **Header placement:** `.txt` artifacts start with `# wieland-corpus: <sha>`; markdown artifacts (`SKILL.md`, per-rule files) carry `<!-- wieland-corpus: <sha> -->` (SKILL.md places it just after its skill frontmatter, so the file stays a valid skill).
- **L0 token budget:** `approx_tokens(s) = ceil(s.len() / 4)` — a deterministic byte/4 approximation, no tokenizer dependency. Documented and cheap; it over- rather than under-counts for ASCII-heavy rule text, which is the safe direction for a budget. Budget = `L0_TOKEN_BUDGET` (2000). `bundle` render returns `CompileError::Budget` (build fails) if `core.txt` exceeds it. Current `core.txt`: ~844 tokens.
- **Drift gate:** `wieland compile --all --check` renders all targets in memory and diffs against committed `dist/`, reporting `changed:` / `missing:` / `stale:` lines and exiting non-zero. Stale-file detection (a `dist/` file no target produced) runs only in `--all` mode; a single-target `--check` compares only that target's files.
- **Outputs:** `dist/all.txt`, `dist/core.txt`, `dist/claude-code/SKILL.md`, `dist/claude-code/rules/<id>.md`. `dist/` is committed (removed from `.gitignore` at M2) so the gate has a baseline to diff against.

### M4 addition — the `deny-set` target + `wieland gate`

- **`deny-set` target** (in `Target::ALL`, so the drift gate covers it) emits `dist/deny-set/`: `lints.toml` (a `[lints.clippy]` table with `<lint> = "deny"`, paste-into-Cargo.toml), `clippy.toml` (config placeholder), and `rulemap.tsv` (`clippy::<lint> → rule_id → title`). Selection: `deny_set(corpus)` = rules with `severity: deny` **and** `enforceability: gate`, taking each rule's `detect.clippy` lints, deduped by lint (first rule in category/id order wins). Currently 11 lints from rules like `anti-unwrap-abuse`, `unsafe-safety-comment`, `anti-lock-across-await`, `lint-deny-correctness`. Verified accepted by a plain `cargo clippy` on a scratch crate.
- **`clippy::panic` is deliberately NOT in the deny-set.** It flags every `panic!`, but `err-expect-bugs-only` legitimises bug-panics, so a blanket deny is wrong. `macro-proc-error-spans` (which had over-claimed the general lints) was corrected to `judge`/tier-1; general `unwrap`/`expect` stay owned by `anti-unwrap-abuse`.
- **`wieland gate [--all|--staged|--changed]`** runs `cargo clippy --message-format=json` in the CWD with `-D clippy::<lint>` for each deny-set lint, parses the JSON diagnostics, maps each back via `clippy::<lint> → rule`, and prints `file:line: [rule_id] title` + a `fix:` line (the rule's `## Rule` statement); non-zero on any violation. It lints lib+bins only (not `--all-targets`), because the rules permit `unwrap`/`expect` in tests. The corpus is found via `WIELAND_RULES` (default `./rules`). `--staged`/`--changed` filter diagnostics to git's staged/changed files by path-suffix match.
- **Dogfood:** `wieland gate --all` on Wieland's own crates is clean and runs in `ci/check.sh`. The one justified `expect` (embedded, compile-time-constant taxonomy) carries `#[allow(clippy::expect_used)]`.
