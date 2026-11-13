# Contributing

## Adding or changing a rule

1. Pick the pack: `rules/` (Rust), `rules/typescript/` or `rules/go/`.
2. Pick a category from the pack's taxonomy (`spec/taxonomy/`); the id must start with its prefix.
3. Fill the frontmatter (`spec/schema/rule.schema.yaml`):
   - `enforceability: gate` only with a real detector: `detect.clippy` (Rust), `detect.oxlint` or
     `detect.tsconfig` (TypeScript), `detect.golangci` (Go). A rule may not carry another language's
     detector.
   - `tier` 2 or 3 needs a detector; tier 0 is the always-on core and has a token budget.
   - `reviewed` is today's date; `sources` names where the advice comes from (`wieland` for original
     rules).
4. Write `## Rule` (one imperative sentence), `## Why` (the cost of getting it wrong), `## Do` and
   `## Don't`. The `Do` example must compile on its own; `Don't` shows the mistake.
5. Run `bash ci/check.sh`. It validates the pack, checks that every detector exists in the pinned
   tools, type-checks the TypeScript and Go examples and runs the gate fixtures.
6. Run `wieland compile --all` so `dist/` matches.

## House rules

- Rust code carries no comments; names and structure explain it. Docs live in `spec/` and Markdown.
- `cargo fmt` and `cargo clippy -- -D warnings` clean; Wieland passes its own gate.
- No invented lints, no examples that do not compile, no claims of enforcement without a detector.
- A new detector tool is pinned to an exact version, with the reason in `PLAN.md`.

## Changing a pinned tool

Bump the version in `tools/typescript/package.json` (and its lockfile) or in `ci/install.sh` (Go),
run `ci/install.sh`, then `wieland check-detectors` and `bash ci/check.sh`. Rules that depend on
changed behaviour get a new `reviewed` date.
