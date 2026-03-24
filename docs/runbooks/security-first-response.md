# Security First Response

## Severity Model

- Critical: active exploit, data exposure risk, or remote compromise
- High: credible privilege escalation or integrity compromise
- Medium: important weakness with limited exploitability
- Low: hygiene issue with minimal impact

## First Hour Actions

1. Triage and confirm signal (do not speculate publicly).
2. Contain exposure path (disable affected workflow where possible).
3. Preserve evidence (logs, repro steps, impacted versions).
4. Assign incident lead and communications owner.
5. Decide: hotfix, rollback, or temporary feature hold.

## Communication Defaults

- Internal update cadence: every 30 minutes while active.
- Include: impact, current status, next action, owner.
- Avoid sharing unverified root-cause statements.

## Exit Criteria

- Containment confirmed
- Remediation patch verified through canonical gates
- Post-incident action items assigned with owners and due dates
