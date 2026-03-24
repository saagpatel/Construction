# Completion Backlog (Execution-Ready)

Last updated: 2026-03-01

## Prioritization rules

- P0: ship blocker (must complete before release).
- P1: high-risk quality or reliability gap.
- P2: important completion item, can follow after ship if needed.

## P0 - Required before release

1. Keep canonical verification contract green in local + CI.
   - Files: `.codex/verify.commands`, `.github/workflows/quality-gates.yml`
   - Done when: all required commands pass in PR validation.
   - Status: Done
2. Keep documentation aligned to real implemented scope.
   - Files: `README.md`, `docs/source-of-truth/*`
   - Done when: no claims conflict with actual command/UI coverage.
   - Status: In progress
3. Finalize release gate and rollback procedure.
   - Files: `docs/release/release-readiness-checklist.md`, `docs/runbooks/rollback-and-recovery.md`
   - Done when: go/no-go checklist is complete and rollback test is documented.
   - Status: In progress

## P1 - Complete immediately after P0

1. Frontend workflow parity gaps:
   - incident edit route and form reuse
   - OSHA 301 UX parity and export clarity
   - JSA approval path and RCA closeout handoff clarity
   - Status: Done
2. Release execution finalization:
   - deployment target/channel decision
   - credential availability for publish pipeline
   - Status: Waiting on PM inputs
3. Expand automated tests for newly hardened command paths.
   - Rust: command/db tests for import validation and toolbox completion constraints.
   - Status: In progress

## P2 - Post-ship hardening queue

1. Full sync/auth operationalization from schema foundations.
2. Role-based authorization enforcement in command handlers.
3. Observability dashboards and alert thresholds for runtime support.

## Dependency notes

- P1 feature expansion should start only after P0 release discipline is stable.
- Any P1 item that changes data shape must include migration compatibility tests.
