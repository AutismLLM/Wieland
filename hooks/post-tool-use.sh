#!/usr/bin/env sh
# Claude Code PostToolUse hook.
#
# Fires after Edit/Write (see hooks/settings.json.sample, matcher
# "Edit|Write"). If the edited file is Rust, runs `wieland gate --changed`
# and surfaces any deny-set violation back to the agent.
#
# Contract (docs.claude.com/en/docs/claude-code/hooks): PostToolUse receives
# a JSON object on stdin (session_id, cwd, tool_name, tool_input, ...) and
# cannot literally block -- the tool already ran. Exiting 2 shows our
# stderr to Claude, which is enough for it to notice and self-correct; a
# clean run exits 0 with no output. (An alternative is printing
# {"decision":"block","reason":"..."} to stdout on exit 0 -- see SPEC.md;
# we use the exit-2 path because it needs no JSON assembly here and is the
# documented mechanism for this event.)
#
# Binary resolution: PATH by default, override with WIELAND_BIN=/path/to/wieland.

set -u

WIELAND_BIN="${WIELAND_BIN:-wieland}"

input=$(cat)

tool_name=""
file_path=""

if command -v jq >/dev/null 2>&1; then
    tool_name=$(printf '%s' "$input" | jq -r '.tool_name // empty' 2>/dev/null)
    file_path=$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
elif command -v python3 >/dev/null 2>&1; then
    tool_name=$(printf '%s' "$input" | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    d = {}
print(d.get("tool_name") or "")
' 2>/dev/null)
    file_path=$(printf '%s' "$input" | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    d = {}
print((d.get("tool_input") or {}).get("file_path") or "")
' 2>/dev/null)
else
    tool_name=$(printf '%s' "$input" | grep -o '"tool_name"[[:space:]]*:[[:space:]]*"[^"]*"' | head -n1 | sed -E 's/.*:[[:space:]]*"([^"]*)"/\1/')
    file_path=$(printf '%s' "$input" | grep -o '"file_path"[[:space:]]*:[[:space:]]*"[^"]*"' | head -n1 | sed -E 's/.*:[[:space:]]*"([^"]*)"/\1/')
fi

# Guard 1: only act on edit-shaped tools. Unknown/unparseable tool_name
# (empty) falls through rather than no-ops, so a parser-less environment
# still gates instead of going silent.
case "$tool_name" in
    Edit | Write | MultiEdit | "") ;;
    *) exit 0 ;;
esac

# Guard 2: only act on .rs files. If file_path couldn't be determined,
# fall through and gate the whole changed set rather than guessing wrong.
if [ -n "$file_path" ]; then
    case "$file_path" in
        *.rs) ;;
        *) exit 0 ;;
    esac
fi

if ! command -v "$WIELAND_BIN" >/dev/null 2>&1; then
    echo "post-tool-use: '$WIELAND_BIN' not found on PATH (set WIELAND_BIN to override); skipping gate" >&2
    exit 0
fi

if [ -z "${WIELAND_RULES:-}" ]; then
    project_dir="${CLAUDE_PROJECT_DIR:-}"
    if [ -n "$project_dir" ] && [ -d "$project_dir/rules" ]; then
        WIELAND_RULES="$project_dir/rules"
        export WIELAND_RULES
    fi
fi

output=$("$WIELAND_BIN" gate --changed 2>&1)
status=$?

if [ "$status" -eq 0 ]; then
    exit 0
fi

printf '%s\n' "$output" >&2
echo "post-tool-use: wieland gate found deny-severity violations in changed .rs files -- fix them (id + why + fix above) before continuing" >&2
exit 2
