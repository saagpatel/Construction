# Backup and Restore Runbook

## Scope

SQLite application data backup and restore for local-first operation.

## Backup Procedure

1. Stop the app to prevent in-flight writes.
2. Copy the SQLite DB file and attachments directory.
3. Store backups with timestamped folder names.
4. Validate backup file readability before archiving.

## Restore Procedure

1. Stop the app.
2. Move current DB/attachments to a `rollback-pre-restore` folder.
3. Copy backup DB/attachments into the runtime data directory.
4. Start the app and run smoke checks:
   - Dashboard loads
   - Incident list loads
   - OSHA page loads

## Restore Validation

- Confirm row counts for incidents and corrective actions.
- Confirm attachment files exist for at least one known incident.
- Confirm no migration errors on app startup.

## RTO/RPO Assumptions

- Target RTO: 30 minutes
- Target RPO: up to last successful backup interval

## Escalation

- Primary: Engineering owner (runtime/data)
- Secondary: Release owner
