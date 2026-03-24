# Release Readiness Checklist

Status key: PASS | FAIL | NOT RUN

## Functional Completion

- [ ] Core routes load and complete expected user journeys.
- [ ] Incident, RCA, OSHA, Toolbox, and JSA baseline flows operate without blocking defects.

## Quality and Testing

- [ ] `pnpm verify` -> PASS
- [ ] CI `quality-gates` -> PASS
- [ ] CI `git-hygiene` -> PASS
- [ ] CI `lockfile-rationale` -> PASS (when lockfiles or `.perf-baselines` change)
- [ ] CI `perf-enforced` -> PASS (for production profile repos)
- [ ] No open P0/P1 defects

## Security and Reliability

- [ ] No known critical security blockers
- [ ] Rollback runbook validated for current release

## Documentation and Comms

- [ ] Source-of-truth docs updated
- [ ] Decision log updated for release-impacting decisions
- [ ] Stakeholder release brief prepared

## Final Gate

- [ ] Go/no-go decision recorded with owner signoff
- [ ] Rollback owner and execution window confirmed
