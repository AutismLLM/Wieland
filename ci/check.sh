#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "== cargo fmt --check =="
cargo fmt --check

echo "== cargo clippy -D warnings =="
cargo clippy --all-targets -- -D warnings

echo "== cargo test =="
cargo test --quiet

echo "== wieland validate rules =="
cargo run --quiet -p wieland-cli -- validate rules

echo "== wieland validate rules/typescript =="
cargo run --quiet -p wieland-cli -- validate rules/typescript

echo "== wieland validate rules/go =="
cargo run --quiet -p wieland-cli -- validate rules/go

echo "== wieland check-detectors (every detector exists in the pinned tools) =="
cargo run --quiet -p wieland-cli -- check-detectors rules

echo "== check-detectors self-test (a fake oxlint rule, compiler option and golangci linter must fail) =="
seed="target/ci-seed"
mkdir -p "$seed/rule/typescript" "$seed/option/typescript"
cp rules/typescript/*.md "$seed/rule/typescript/"
cp rules/typescript/*.md "$seed/option/typescript/"
sed -i 's|typescript/no-explicit-any|typescript/no-explicit-anything|' "$seed/rule/typescript/type-no-explicit-any.md"
sed -i 's|  tsconfig: \[erasableSyntaxOnly\]|  tsconfig: [erasableSyntaxOnlyy]|' "$seed/option/typescript/syn-erasable-only.md"
mkdir -p "$seed/linter/go"
cp rules/go/*.md "$seed/linter/go/"
sed -i 's|golangci: \[nilnil\]|golangci: [nilnill]|' "$seed/linter/go/err-no-nil-nil.md"
for case in rule option linter; do
    if cargo run --quiet -p wieland-cli -- check-detectors "$seed/$case" >/dev/null 2>&1; then
        echo "FAIL: check-detectors accepted a seeded fake $case"
        exit 1
    fi
    echo "ok: seeded fake $case rejected"
done

echo "== check-examples --pack typescript (every TypeScript Do block type-checks) =="
cargo run --quiet -p wieland-cli -- check-examples --pack typescript

echo "== check-examples --pack go (every Go Do block passes go vet) =="
cargo run --quiet -p wieland-cli -- check-examples --pack go

echo "== wieland compile --all --check (drift gate) =="
if [ ! -d dist ]; then
    echo "dist/ absent (fresh clone): compiling it once, then checking"
    cargo run --quiet -p wieland-cli -- compile --all
fi
cargo run --quiet -p wieland-cli -- compile --all --check

echo "== wieland gate --all (dogfood: Wieland passes its own deny-set) =="
cargo run --quiet -p wieland-cli -- gate --all

echo "== gate integration (foundation blocks bad code, passes good) =="
cargo build --quiet -p wieland-cli
root="$(pwd)"
wieland="$root/target/debug/wieland"
rules="$root/rules"

bad_out="$(cd ci/fixtures/gate-bad && WIELAND_RULES="$rules" "$wieland" gate --all 2>&1)" && bad_ok=0 || bad_ok=1
if [ "$bad_ok" -ne 1 ]; then
    echo "FAIL: gate passed on the known-bad fixture"
    echo "$bad_out"
    exit 1
fi
for rid in anti-unwrap-abuse anti-lock-across-await lint-deny-correctness; do
    if ! grep -q "$rid" <<<"$bad_out"; then
        echo "FAIL: expected rule '$rid' in gate output for gate-bad"
        echo "$bad_out"
        exit 1
    fi
done
echo "ok: gate-bad blocked ($(grep -c 'fix:' <<<"$bad_out") violations, expected rules present)"

if ! (cd ci/fixtures/gate-good && WIELAND_RULES="$rules" "$wieland" gate --all); then
    echo "FAIL: gate flagged the known-good fixture"
    exit 1
fi
echo "ok: gate-good clean"

echo "== TypeScript gate integration =="
ts_bad="$(cd ci/fixtures/gate-ts-bad && WIELAND_RULES="$rules" "$wieland" gate --all 2>&1)" && ts_bad_ok=0 || ts_bad_ok=1
if [ "$ts_bad_ok" -ne 1 ]; then
    echo "FAIL: gate passed on gate-ts-bad"
    echo "$ts_bad"
    exit 1
fi
for rid in cfg-strict-compiler-flags cfg-no-ts-suppression type-no-non-null-assertion type-no-explicit-any type-exhaustive-switch async-no-floating-promise; do
    if ! grep -q "\[$rid\]" <<<"$ts_bad"; then
        echo "FAIL: expected rule '$rid' in gate output for gate-ts-bad"
        echo "$ts_bad"
        exit 1
    fi
done
echo "ok: gate-ts-bad blocked ($(grep -c 'fix:' <<<"$ts_bad") violations, expected rules present)"

ts_file="$(cd ci/fixtures/gate-ts-bad && WIELAND_RULES="$rules" "$wieland" gate --file src/bad.ts 2>&1)" || true
if ! grep -q "\[async-no-floating-promise\]" <<<"$ts_file"; then
    echo "FAIL: gate --file src/bad.ts did not report the floating promise"
    echo "$ts_file"
    exit 1
fi
echo "ok: gate-ts-bad --file scope reports the file's violations"

if ! (cd ci/fixtures/gate-ts-good && WIELAND_RULES="$rules" "$wieland" gate --all); then
    echo "FAIL: gate flagged gate-ts-good"
    exit 1
fi
echo "ok: gate-ts-good clean"

echo "== hook integration (Rust) =="
fixtures="$root/ci/fixtures"
rs_hook="$(printf '{"tool_name":"Edit","tool_input":{"file_path":"%s"}}' "$fixtures/gate-bad/src/lib.rs" | WIELAND_RULES="$rules" "$wieland" hook 2>&1)" && rs_rc=0 || rs_rc=$?
if [ "$rs_rc" -ne 2 ] || ! grep -q "\[anti-unwrap-abuse\]" <<<"$rs_hook"; then
    echo "FAIL: hook did not block gate-bad/src/lib.rs with exit 2 (got $rs_rc)"
    echo "$rs_hook"
    exit 1
fi
echo "ok: hook blocks gate-bad/src/lib.rs (exit 2)"
if ! printf '{"tool_name":"Edit","tool_input":{"file_path":"%s"}}' "$fixtures/gate-good/src/lib.rs" | WIELAND_RULES="$rules" "$wieland" hook; then
    echo "FAIL: hook blocked gate-good"
    exit 1
fi
echo "ok: hook passes gate-good/src/lib.rs"

echo "== hook integration (TypeScript) =="
fixtures="$root/ci/fixtures"
hook_out="$(printf '{"tool_name":"Edit","tool_input":{"file_path":"%s"}}' "$fixtures/gate-ts-bad/src/bad.ts" | WIELAND_RULES="$rules" "$wieland" hook 2>&1)" && hook_rc=0 || hook_rc=$?
if [ "$hook_rc" -ne 2 ] || ! grep -q "\[async-no-floating-promise\]" <<<"$hook_out"; then
    echo "FAIL: hook did not block gate-ts-bad with exit 2 (got $hook_rc)"
    echo "$hook_out"
    exit 1
fi
echo "ok: hook blocks gate-ts-bad/src/bad.ts (exit 2)"
if ! printf '{"tool_name":"Edit","tool_input":{"file_path":"%s"}}' "$fixtures/gate-ts-good/src/good.ts" | WIELAND_RULES="$rules" "$wieland" hook; then
    echo "FAIL: hook blocked gate-ts-good"
    exit 1
fi
echo "ok: hook passes gate-ts-good/src/good.ts"

echo "== Go gate and hook integration =="
go_bad="$(cd ci/fixtures/gate-go-bad && WIELAND_RULES="$rules" "$wieland" gate --all 2>&1)" && go_bad_ok=0 || go_bad_ok=1
if [ "$go_bad_ok" -ne 1 ]; then
    echo "FAIL: gate passed on gate-go-bad"
    echo "$go_bad"
    exit 1
fi
for rid in err-check-every-error err-no-nil-after-check err-wrap-with-w type-checked-type-assertion res-close-http-body ctx-request-with-context; do
    if ! grep -q "\[$rid\]" <<<"$go_bad"; then
        echo "FAIL: expected rule '$rid' in gate output for gate-go-bad"
        echo "$go_bad"
        exit 1
    fi
done
echo "ok: gate-go-bad blocked ($(grep -c 'fix:' <<<"$go_bad") violations, expected rules present)"
if ! (cd ci/fixtures/gate-go-good && WIELAND_RULES="$rules" "$wieland" gate --all); then
    echo "FAIL: gate flagged gate-go-good"
    exit 1
fi
echo "ok: gate-go-good clean"
hook_out="$(printf '{"tool_name":"Write","tool_input":{"file_path":"%s"}}' "$fixtures/gate-go-bad/bad.go" | WIELAND_RULES="$rules" "$wieland" hook 2>&1)" && hook_rc=0 || hook_rc=$?
if [ "$hook_rc" -ne 2 ] || ! grep -q "\[err-check-every-error\]" <<<"$hook_out"; then
    echo "FAIL: hook did not block gate-go-bad with exit 2 (got $hook_rc)"
    echo "$hook_out"
    exit 1
fi
echo "ok: hook blocks gate-go-bad/bad.go (exit 2)"
if ! printf '{"tool_name":"Write","tool_input":{"file_path":"%s"}}' "$fixtures/gate-go-good/good.go" | WIELAND_RULES="$rules" "$wieland" hook; then
    echo "FAIL: hook blocked gate-go-good"
    exit 1
fi
echo "ok: hook passes gate-go-good/good.go"

echo "== agent adapters (documented payloads per agent) =="
cursor_out="$(printf '{"tool_name":"Write","tool_input":"{\\"file_path\\":\\"%s\\"}"}' "$fixtures/gate-go-bad/bad.go" | WIELAND_RULES="$rules" "$wieland" hook --agent cursor)"
if ! python3 -c 'import json,sys; d=json.loads(sys.argv[1]); assert "[err-check-every-error]" in d["additional_context"]' "$cursor_out"; then
    echo "FAIL: cursor adapter did not return additional_context with the Go finding"
    echo "$cursor_out"
    exit 1
fi
echo "ok: cursor postToolUse -> additional_context"
codex_rc=0
python3 -c 'import json,sys; print(json.dumps({"tool_name":"apply_patch","cwd":sys.argv[1],"tool_input":{"command":"*** Begin Patch\n*** Update File: gate-ts-bad/src/bad.ts\n*** End Patch"}}))' "$fixtures" | WIELAND_RULES="$rules" "$wieland" hook --agent codex >/dev/null 2>&1 || codex_rc=$?
if [ "$codex_rc" -ne 2 ]; then
    echo "FAIL: codex adapter did not block an apply_patch to gate-ts-bad (got $codex_rc)"
    exit 1
fi
echo "ok: codex apply_patch -> exit 2"
gemini_out="$(printf '{"tool_name":"replace","tool_input":{"file_path":"%s"}}' "$fixtures/gate-bad/src/lib.rs" | WIELAND_RULES="$rules" "$wieland" hook --agent gemini)"
if ! python3 -c 'import json,sys; d=json.loads(sys.argv[1]); assert "[anti-unwrap-abuse]" in d["hookSpecificOutput"]["additionalContext"]' "$gemini_out"; then
    echo "FAIL: gemini adapter did not return additionalContext with the Rust finding"
    echo "$gemini_out"
    exit 1
fi
gemini_quiet="$(printf '{"tool_name":"write_file","tool_input":{"file_path":"/tmp/notes.md"}}' | "$wieland" hook --agent gemini)"
if [ "$gemini_quiet" != "{}" ]; then
    echo "FAIL: gemini adapter must print {} when there is nothing to report (got: $gemini_quiet)"
    exit 1
fi
echo "ok: gemini AfterTool -> additionalContext, {} when quiet"
pi_out="$(WIELAND_BIN="$wieland" WIELAND_RULES="$rules" node hooks/pi/stub-test.ts "$fixtures/gate-ts-bad/src/bad.ts")"
if ! python3 -c 'import json,sys; d=json.loads(sys.argv[1]); assert d["patch"]["isError"] is True and "[async-no-floating-promise]" in d["patch"]["content"][1]["text"]' "$pi_out"; then
    echo "FAIL: pi extension did not append the finding to the edit result"
    echo "$pi_out"
    exit 1
fi
echo "ok: pi tool_result extension appends findings"

echo "== project checks (wieland.yaml) =="
pc_out="$(printf '{"tool_name":"Write","tool_input":{"file_path":"%s"}}' "$fixtures/project-bad/src/note.md" | "$wieland" hook 2>&1)" && pc_rc=0 || pc_rc=$?
if [ "$pc_rc" -ne 2 ] || ! grep -q "\[project:no-forbidden\]" <<<"$pc_out"; then
    echo "FAIL: a failing project check did not block the edit (got $pc_rc)"
    echo "$pc_out"
    exit 1
fi
if ! printf '{"tool_name":"Write","tool_input":{"file_path":"%s"}}' "$fixtures/project-good/src/note.md" | "$wieland" hook; then
    echo "FAIL: a passing project check blocked the edit"
    exit 1
fi
echo "ok: project check blocks project-bad, passes project-good"

echo "== all checks passed =="
