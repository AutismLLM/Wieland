#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
remotes="${1:?usage: ci/publish-public.sh <remote[,remote...]> [message]}"
message="${2:-Sync from the development repository}"
IFS=, read -r -a remote_list <<<"$remotes"
remote="${remote_list[0]}"
pattern="${WIELAND_PRIVATE_PATTERN:-}"

if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "publish: commit your changes first"
    exit 1
fi

if [ -n "$pattern" ] && git grep -n -I -i -E "$pattern" HEAD -- . >/dev/null; then
    echo "publish: private names found in HEAD, not publishing:"
    git grep -n -I -i -E "$pattern" HEAD -- . | head -20
    exit 1
fi

git fetch -q "$remote"
parent="$(git rev-parse "$remote/main")"
tree="$(git rev-parse 'HEAD^{tree}')"
if [ "$(git rev-parse "$parent^{tree}")" = "$tree" ]; then
    echo "publish: $remote/main already has this tree"
    exit 0
fi
commit="$(git commit-tree "$tree" -p "$parent" -m "$message")"
for target in "${remote_list[@]}"; do
    git push "$target" "$commit:refs/heads/main"
done
echo "publish: ${remotes} main -> $commit (one snapshot commit on top of the public history)"
