# Rollback and Recovery Runbook

## Rollback Triggers

- Critical regression in incident management flow.
- Data integrity risk in SQLite migrations or write paths.
- Repeated CI failures on release candidate branch with no same-day fix.

## Rollback Procedure

1. Stop release progression.
2. Revert the smallest known-bad change set.
3. Re-run `pnpm verify`.
4. Validate critical journeys manually:
   - Create incident
   - Load incident detail
   - RCA session create/update
   - OSHA 300/300A load
5. Reopen release only when all gates are green.

## Recovery After Rollback

1. Record incident in decision log.
2. Create follow-up task with acceptance test to prevent recurrence.
3. Re-validate release checklist before reattempt.
