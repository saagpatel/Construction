# Go / No-Go Checklist

## Functional Completion

- [ ] Core runtime workflows verified: incidents, OSHA, RCA, toolbox, JSA, settings/import
- [ ] No unresolved P0/P1 defects

## Quality and Testing

- [ ] `bash .codex/scripts/run_verify_commands.sh` passed
- [ ] No known flaky blockers in required gates

## Performance

- [ ] Bundle/build/memory checks pass against baseline policy
- [ ] Any baseline change is documented and reviewed

## Security and Reliability

- [ ] No unresolved critical/high security blockers
- [ ] Rollback runbook validated for this release

## Release Operations

- [ ] Release notes prepared
- [ ] Owner on-call/escalation coverage confirmed
- [ ] Deployment target and rollback owner confirmed

## Decision

- [ ] GO
- [ ] NO-GO
- Decision owner:
- Timestamp:
- Notes:
