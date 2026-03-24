# Release and Rollback Runbook

## Purpose

Provide a repeatable path to ship safely and recover quickly.

## Pre-Release Checklist

1. Confirm canonical verify contract is green.
2. Confirm PR template sections are complete (`What`, `Why`, `How`, `Testing`, `Performance impact`, `Risk / Notes`).
3. Confirm lockfile rationale section if lockfile changed.
4. Confirm release notes draft and owner assignment.

## Release Procedure

1. Freeze release candidate branch content.
2. Run canonical verify from clean workspace.
3. Validate packaging artifacts via Tauri build.
4. Execute go/no-go checklist in `docs/release/go-no-go-checklist.md`.
5. Proceed with deploy only if all blocking gates are green.

## Rollback Triggers

- P0/P1 defect discovered post-deploy
- Data integrity or migration regression
- Crash/reliability threshold breach
- Security incident requiring containment

## Rollback Steps

1. Announce rollback decision and incident owner.
2. Revert to previous known-good release artifact.
3. Validate app startup, key workflows, and data access on rollback version.
4. Record root cause and corrective action owner.
5. Re-open release only after corrective patch passes full verify.
