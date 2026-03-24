# Verification Matrix

Last updated: 2026-03-01

## Canonical Local Verification

Run:

```bash
bash .codex/scripts/run_verify_commands.sh
```

This executes `.codex/verify.commands` in order:

1. `pnpm git:guard:all`
2. `pnpm check:frontend`
3. `pnpm lint:rust`
4. `pnpm test:rust`
5. `pnpm perf:build`
6. `pnpm perf:bundle`
7. `pnpm perf:assets`
8. `pnpm perf:memory`
9. `pnpm perf:summary`

## CI Gate Mapping

- `quality-gates.yml`
  - Runs frontend and Rust quality checks (`pnpm check:frontend`, `pnpm lint:rust`, `pnpm test:rust`)
- `git-hygiene.yml`
  - Enforces branch naming, commit/PR semantics, and secret scanning
- `lockfile-rationale.yml`
  - Enforces non-empty lockfile rationale and baseline governance sections when needed
- `perf-foundation.yml`
  - Runs baseline performance checks for all PRs
- `perf-enforced.yml`
  - Applies production-profile performance enforcement and threshold comparison
- `deploy.yml`
  - Runs release-candidate verify and packages release artifacts

## Release Gate Rule

A release candidate is considered quality-eligible only when:

- Local canonical verification passes.
- Required CI workflows are green.
- `docs/release/release-readiness-checklist.md` is marked PASS.
