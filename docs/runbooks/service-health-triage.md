# Service Health Triage Runbook

## Trigger

Use this runbook when the app fails to build, fails quality gates, or shows critical runtime failure in core user flows.

## First 15 Minutes

1. Confirm scope: local only, branch only, or reproducible across clean checkout.
2. Run canonical verification:
   - `pnpm verify`
3. Identify failing gate category:
   - Type/build
   - Rust fmt/lint/test
   - Perf guard
   - Runtime flow issue

## Containment

- Block merge while any P0/P1 failure is active.
- If release candidate exists, freeze new changes outside the failing area.

## Diagnosis Path

1. Reproduce with minimal steps.
2. Identify first bad change.
3. Apply smallest safe fix.
4. Re-run full verify.

## Exit Conditions

- Local checks passed.
- CI quality gates passed.
- Root cause and fix documented in decision log.
