# Production Release Notes - 2026-03-01

## Release

- Version: 1.0.0-rc.2
- Date: 2026-03-01
- Owner: Release lane

## What changed

- Hardened backend reliability:
  - Safer incident date handling during create flow
  - CSV import now validates date and numeric fields with row-level errors
  - Dashboard aggregate query now handles empty-result sets safely
  - Toolbox attendee deletion now returns not-found on missing IDs
  - Toolbox completion now requires signed attendees before completion
  - Added OSHA 300A/301 CSV export commands for report parity
- Hardened data migration safety:
  - Each migration now runs and records inside one transaction
- Closed remaining no-credential UX parity items:
  - Incident edit route (`/incidents/:id/edit`) and UI form
  - OSHA 301 report tab now exposes incident-level report view
  - JSA page now supports review/approval/in-progress/completed transitions
  - RCA page now supports direct incident status handoff (open -> in review -> closed)
- Strengthened release/quality controls:
  - Local verify now uses `pnpm check:frontend` for CI parity
  - Performance summary now marks missing required metrics as `not-run`
  - Asset checks now include both source and built asset directories
  - Lockfile/baseline governance checks now enforce non-empty rationale sections
  - Added release-candidate deploy workflow (`.github/workflows/deploy.yml`)
- Expanded completion documentation:
  - Added verification matrix, onboarding checklists, troubleshooting, and communication templates

## Why this release

- Reduce hidden runtime failures and quality drift between local and CI checks.
- Improve readiness for autonomous execution and production release governance.

## Risk assessment

- Known risks:
  - Some roadmap features remain schema-only and are intentionally deferred.
  - Deployment target provider decision remains external to this repo.
- Mitigations:
  - Release checklist and go/no-go gates updated.
  - Rollback and security runbooks in place.

## Migration/config impact

- Data migration changes:
  - Migration execution transaction handling improved; no schema delta introduced.
- Config/env changes:
  - `PERF_PROFILE` must be explicitly set to `production` for perf-enforced workflow.

## Rollback

- Trigger conditions:
  - Regressions in incident creation/import, toolbox completion flow, or verify gates.
- Rollback path:
  - Revert release branch and follow `docs/runbooks/rollback-and-recovery.md`.
- Data checks after rollback:
  - Verify incident list, toolbox attendees, and migration table integrity.

## Verification evidence

- Canonical verify run: local pass
- CI status: pending PR execution
- Additional release checks:
  - Rust clippy/test pass
  - Frontend typecheck/build pass
