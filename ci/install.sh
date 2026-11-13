#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
root="$(pwd)"
tools="$HOME/.wieland/tools"

if [ "${1:-}" = "--skills-only" ]; then
    skills_only=1
else
    skills_only=0
fi

if [ "$skills_only" -eq 0 ]; then
echo "== build release (niced, capped) =="
nice -n 19 ionice -c3 cargo build --release --quiet --jobs "${WIELAND_BUILD_JOBS:-6}"

echo "== install binaries -> ~/.cargo/bin =="
mkdir -p "$HOME/.cargo/bin"
cp target/release/wieland "$HOME/.cargo/bin/wieland"
cp target/release/wieland-mcp "$HOME/.cargo/bin/wieland-mcp"

echo "== install corpus -> ~/.wieland/rules =="
./target/release/wieland install

echo "== compile artifacts =="
./target/release/wieland compile --all

if command -v npm >/dev/null 2>&1; then
    echo "== pinned TypeScript tools -> $tools/typescript =="
    mkdir -p "$tools/typescript"
    cp tools/typescript/package.json tools/typescript/package-lock.json "$tools/typescript/"
    (cd "$tools/typescript" && npm ci --no-audit --no-fund --loglevel=error)
    "$tools/typescript/node_modules/.bin/tsc" --version
    echo "oxlint $("$tools/typescript/node_modules/.bin/oxlint" --version | tr -d "\n" | sed "s/Version: //")"
else
    echo "== npm not found: skipping TypeScript tools (TypeScript gate will report infra failure) =="
fi

fi

if command -v go >/dev/null 2>&1; then
    echo "== pinned Go tools -> $tools/go/bin =="
    mkdir -p "$tools/go/bin"
    want="2.15.0"
    have="$("$tools/go/bin/golangci-lint" version 2>/dev/null | sed -n 's/.*has version \([0-9.]*\).*/\1/p' || true)"
    if [ "$have" != "$want" ]; then
        (cd /tmp && GOBIN="$tools/go/bin" GOFLAGS=-mod=mod nice go install "github.com/golangci/golangci-lint/v2/cmd/golangci-lint@v$want")
    fi
    "$tools/go/bin/golangci-lint" version
else
    echo "== go not found: skipping Go tools (Go gate will report infra failure) =="
fi

install_skill() {
    local from="$1" name="$2"
    local dest="$HOME/.claude/skills/$name"
    local addendum="$HOME/.wieland/local/skills/$name.md"
    echo "== install Claude skill -> ~/.claude/skills/$name =="
    mkdir -p "$dest"
    cp -r "$from"/. "$dest/"
    if [ -f "$addendum" ]; then
        cat "$addendum" >> "$dest/SKILL.md"
        echo "   + local addendum $addendum"
    fi
}

install_skill dist/claude-code wieland-rust
if [ -d dist/typescript/claude-code ]; then
    install_skill dist/typescript/claude-code wieland-typescript
fi
if [ -d dist/go/claude-code ]; then
    install_skill dist/go/claude-code wieland-go
fi
for skill in skills/*/; do
    [ -d "$skill" ] || continue
    install_skill "$skill" "$(basename "$skill")"
done

if [ "$skills_only" -eq 1 ]; then
    echo "== skills installed (binaries, corpus and tools untouched) =="
    exit 0
fi

echo "== installed =="
"$HOME/.cargo/bin/wieland" validate "$HOME/.wieland/rules"
cd "$root"
