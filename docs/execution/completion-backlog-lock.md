# Completion Backlog Lock

## Critical Path Backlog (Locked)

### B1. Deterministic quality baseline

- Scope: Typecheck, Rust lint/tests, perf checks, git hygiene integrated into canonical verify.
- Acceptance: `bash .codex/scripts/run_verify_commands.sh` passes locally and in CI.
- Status: Done

### B2. CI parity and release gating

- Scope: CI workflows enforce the same release criteria as local verify (quality, hygiene, lockfile/baseline governance, perf).
- Acceptance: Required CI workflows are explicitly mapped and pass for release PRs.
- Status: Done

### B3. Runtime flow hardening

- Scope: Close reliability risks in incident import, dashboard aggregates, toolbox completion, and migration atomicity.
- Acceptance: contract-safe behavior with test coverage and green quality gates.
- Status: Done (current hardening batch)

### B4. Release operating docs

- Scope: Go/no-go checklist, rollback runbook, and security first-response documentation.
- Acceptance: Docs exist and are linked from source-of-truth index.
- Status: Done

### B5. Truth-in-scope documentation

- Scope: README and source docs clearly separate runtime features from schema-only foundations.
- Acceptance: No ambiguous “implemented” wording for non-runtime modules.
- Status: In progress (remaining route-level gaps tracked below)

## Remaining Pre-Ship Completion Items (Locked)

- R1. Incident edit route and UI form reuse (`/incidents/:id/edit`) for full frontend CRUD parity. - Done
- R2. OSHA page parity between tabs and labels (current `301` tab is annual stats, not incident report view). - Done
- R3. Export UX parity across OSHA forms or explicit in-product messaging where export is intentionally limited. - Done
- R4. JSA approval UX completion and explicit closeout signal in RCA-to-incident workflow handoff. - Done
- R5. Final release target decision (distribution channel and credentials) for production publish.

## Deferred Scope (Post-Release Queue)

- UI + command workflows for inspections, near-miss, training, and equipment
- Runtime auth/RBAC/session enforcement and sync execution
- Cloud sync and multi-device conflict workflows
