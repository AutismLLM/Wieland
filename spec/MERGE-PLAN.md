# Wieland — Merge Plan (the "later" step)

This scaffold defines the target shape. Merging is the act of pouring the two vendored sources into `rules/*.md` against `schema/rule.schema.yaml`. Deferred by design — do it as its own reviewed pass, not mixed into framework work.

---

## Phase 0 — pre-flight
- Confirm upstream licenses permit derivative + redistribution (`sources/PROVENANCE.md`).
- Freeze the schema and taxonomy. Any change here after merge starts means re-touching every rule.

## Phase 1 — adopt rust-skills (the easy 90%)
rust-skills is already one-file-per-rule with compatible ids. For each `../rust-skills/rules/<x>.md`:
1. map its category prefix to a `taxonomy/categories.yaml` key;
2. wrap its content in canonical frontmatter (`id`, `title`, `category`, `severity`, `tier`, `sources: [leonardomso/rust-skills#<x>]`, `status: draft`);
3. normalize the body to `## Rule / Why / Do / Don't / Notes`;
4. set `tier` from severity + detector availability (has a clippy lint → tier 2 candidate).

This is mechanical and parallelizable — one rule per unit of work. A good fan-out job: N agents each transform a category, validate against schema, emit to `rules/`.

## Phase 2 — fold in Microsoft guidelines (the judgement 10%)
MS material is prose, not atomic. For each guideline:
- **new idea** neither source has → author a `wieland`-tagged rule;
- **same idea as a rust-skills rule** → merge: enrich the `Why`, add `microsoft/rust-guidelines#<anchor>` to `sources:`, record the merge;
- **pure rationale / long-form** → route to `research/<slug>.md`, link from the relevant rule's `Notes`, do not force it into a rule.

## Phase 3 — detectors & tiers
- For every rule, fill `detect.clippy` where a lint exists (this is the L2/L3 unlock).
- Promote the true non-negotiables to **tier 0** (target: 15–25 rules, `core.txt` < ~2k tokens).
- Assign the `deny` set (tier-3 gate) — only rules with a reliable non-grep detector.

## Phase 4 — validate & compile
- `wieland validate` — schema-clean corpus, no dup ids, every category resolves.
- `wieland compile bundle` (and each harness target) — exports build and are byte-stable.
- Spot-check: pick 10 rules, confirm each renders correctly into Claude SKILL, Cursor `.mdc`, and `all.txt`.

## Phase 5 — wire the layers
- Stand up `wieland-forge` MCP (stdio first, then HTTPS).
- Install the Claude Code hooks; dry-run the gate on a real Rust workspace (client, gateway, embedder).
- Publish the HTTPS research endpoint internally.

---

## Quality bar (applies to every merged rule)
- Has a real `Do` and `Don't` in compiling-shape Rust.
- `Why` states a concrete cost, not a platitude.
- Edition-honest (`rust_edition`, `msrv` where relevant).
- Provenance complete. No orphan rules.
- Passes the house slop gate: no filler, no hedging, no invented lints.

## Build order (Fosite-corrected — do NOT build all emitters up front)

The risk is inverting the order: 6 export formats specced before a single real rule exists means rewriting every emitter the moment the frontmatter schema meets reality. Sequence instead:

1. **Spike 20 rules** spanning all 4 tiers, all enforceability classes, and *both* sources. Small enough to throw away.
2. **Freeze the schema** against those 20. This is when the schema earns its shape — not before.
3. **Compiler with exactly 2 targets:** `bundle` (all.txt/core.txt) + `claude-code`. Prove the projection and the drift gate on the smallest useful set.
4. **Drift gate first** (`wieland compile --all --check`) — before target #3 even. It is the one real failure mode; nothing else ships until it exists.
5. **Merge the remaining ~245 rules** (the Phase-1 fan-out below).
6. **Add other emitters one at a time, only when a harness is actually in use.** Cursor `.mdc`, Copilot, Aider, Codex are **SPECULATIVE** targets — NOTE them, do not build them until a real consumer needs one. (Building for KNOWN only.)

## Suggested execution
The Phase-1 mechanical transform (step 5 above) is ideal for a workflow fan-out (one agent per category, schema-validated output, adversarial second pass for body quality). Phases 2–3 (MS fold-in, detector/tier assignment) need human/Claude judgement and should not be blindly delegated. Nothing here runs until you green-light the merge.
