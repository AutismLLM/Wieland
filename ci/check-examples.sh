#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "== wieland check-examples (compile-check every Do block: Rust, TypeScript, Go) =="
echo "Heavy: builds a throwaway crate with ~40 deps under target/check-examples/."
echo "Kept out of ci/check.sh; run in a separate lane or before a corpus release."
cargo run --quiet -p wieland-cli -- check-examples
