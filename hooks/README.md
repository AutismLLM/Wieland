# Wieland hooks: every agent, every edit

`wieland hook` gates the file an agent just wrote. It picks the language from the extension
(`.rs`, `.ts/.tsx/.mts/.cts`, `.go`), walks up to the project manifest (`Cargo.toml`,
`tsconfig.json`, `go.mod`), runs the same gate as `wieland gate`, keeps the findings for the edited
file and answers in the agent's own format. A git `pre-commit` hook is the backstop for anything
that edits code without a hook: other agents, editors, people.

| Agent | Config file | Event | How findings reach the model | Template |
|---|---|---|---|---|
| Claude Code | `~/.claude/settings.json` or `.claude/settings.json` | `PostToolUse` on `Edit\|Write\|MultiEdit` | stderr, exit 2 | `claude-code/settings.json` |
| Codex CLI | `~/.codex/hooks.json` or `.codex/hooks.json` | `PostToolUse` on `apply_patch\|Edit\|Write` | stderr, exit 2; paths read from the patch text | `codex/hooks.json` |
| Cursor | `~/.cursor/hooks.json` or `.cursor/hooks.json` | `postToolUse` | `{"additional_context": …}` on stdout | `cursor/hooks.json` |
| Gemini CLI | `~/.gemini/settings.json` or `.gemini/settings.json` | `AfterTool` on `write_file\|replace` | `hookSpecificOutput.additionalContext`; `{}` when quiet | `gemini/settings.json` |
| Pi | `~/.pi/agent/extensions/` or `.pi/extensions/` | `tool_result` for `edit`/`write` | appended to the tool result, `isError` on a block | `pi/wieland.ts` |
| anything else | `.git/hooks/pre-commit` | commit | blocks the commit | `pre-commit` |

`wieland hook --agent <claude|codex|cursor|gemini|pi>` selects the reply format; `--path <file>`
gates a file without reading hook JSON (the Pi extension uses it). Input is read from
`tool_input.file_path`, `tool_input.path`, a JSON-string `tool_input` (Cursor), a top-level
`file_path` (Cursor `afterFileEdit`) or the `*** Add/Update File:` lines of an `apply_patch`
(Codex).

## Rules every adapter keeps

- Only findings in the edited file block. Project-level findings (a missing required compiler
  option) come back as advice and block in `wieland gate --all`, `pre-commit` and CI.
- Infrastructure failures never block an edit: no corpus, missing pinned tools, a tool that did not
  start. The hook warns on stderr and exits 0.
- Nothing is written into the project. Compiled configs go to the system temp directory.

## Install

1. `bash ci/install.sh` puts `wieland` on `PATH`, the corpus in `~/.wieland/rules`, the pinned
   TypeScript and Go tools in `~/.wieland/tools`, and the skills in `~/.claude/skills`.
2. Merge the template for each agent you use into its config file (user level to cover every
   project, project level to share with a team).
3. For Pi, copy `pi/wieland.ts` into `~/.pi/agent/extensions/` and run `/reload`.
4. Per repository: `cp hooks/pre-commit .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit`.

## What was verified, and how

`ci/check.sh` feeds each adapter the payload its documentation describes (Claude Code, Codex
`apply_patch`, Cursor `postToolUse` with a string `tool_input`, Gemini `AfterTool`) and runs the Pi
extension against a stub `ExtensionAPI` (`pi/stub-test.ts`). Claude Code is also validated live.
Codex, Cursor, Gemini CLI and Pi are not installed on the build machine, so a live run of each is
still open; open issue to watch: Codex 0.136.0 did not fire `PostToolUse` for `apply_patch`
(openai/codex#26729), where `pre-commit` remains the backstop.

## Older files

`post-tool-use.sh` and `settings.json.sample` are the first Claude Code wiring (a shell wrapper
around `gate --changed`). `wieland hook` replaces both; they stay for reference.
