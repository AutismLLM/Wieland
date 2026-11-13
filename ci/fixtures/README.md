# Gate fixtures

Two throwaway crates that prove `wieland gate` actually enforces the foundation —
not just that Wieland's own code is clean.

- `gate-bad/` — deliberate foundational violations (`unwrap`/`expect`, a lock guard
  held across `.await`, an imprecise `PI` constant, a `x == x` tautology). The gate
  must FAIL here and name `anti-unwrap-abuse`, `anti-lock-across-await`,
  `lint-deny-correctness`.
- `gate-good/` — the same shapes written correctly. The gate must PASS here.

Each crate declares its own empty `[workspace]` so the root workspace build, clippy,
fmt, and test never touch the intentional bad code. `ci/check.sh` runs both.
