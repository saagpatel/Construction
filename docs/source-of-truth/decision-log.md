# Decision Log

## 2026-03-01

### D-001: Align quality gates to one canonical verify contract

- Decision: Expand `.codex/verify.commands` to include typecheck, build, Rust lint/tests, and perf checks.
- Why: Local and CI gate drift was creating false confidence.
- Impact: One command (`pnpm verify`) now reflects release gating expectations.

### D-002: Treat schema-only modules as planned, not shipped

- Decision: Update README and source-of-truth docs to clearly mark inspections/near-miss/training/equipment workflows as pending app-layer implementation.
- Why: Schema presence alone does not equal production feature availability.
- Impact: Reduced scope confusion and cleaner backlog prioritization.

### D-003: Add CI parity workflow for canonical verify

- Decision: Keep CI gates split by concern (`quality-gates`, `git-hygiene`, `lockfile-rationale`, `perf-*`) and align local verify commands to match.
- Why: Different gate categories have different triggers but must still remain logically aligned.
- Impact: Better transparency of failing gate category while preserving local/CI parity.

### D-004: Fail closed when production perf profile is not configured

- Decision: `perf-enforced.yml` now validates `PERF_PROFILE=production` before any perf job runs.
- Why: Silent skipping of production perf enforcement can hide regressions.
- Impact: Production-profile repos now fail fast when enforcement configuration is missing.

### D-005: Harden import and toolbox behavior at contract boundaries

- Decision: Import now validates date and numeric fields per row, and toolbox completion requires attendee signatures.
- Why: These paths previously allowed silent bad data or premature workflow completion.
- Impact: More predictable runtime behavior and fewer compliance-facing edge-case failures.
