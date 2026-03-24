# PM Closeout Workstream Plan (Production-Ready Completion)

## 1) Goal

Close the project with a complete docs/comms package that supports safe production operation, repeatable release execution, and clear stakeholder alignment.

## 2) Scope

This plan covers:

- Source-of-truth documentation
- Operator runbooks
- Onboarding and setup documentation
- Release notes and pull request template alignment
- Stakeholder communication artifacts

This plan does not cover feature implementation work.

## 3) Workstream Outcomes

- Teams can identify the current production truth in one place.
- Operators can handle common incidents and rollbacks without guesswork.
- New contributors can set up and contribute with minimal support.
- Release communication is consistent across PRs, notes, and stakeholder updates.
- Production-ready completion can be signed off using objective done criteria.

## 4) Deliverables (Exact Artifacts)

All paths are repo-relative.

| ID  | Category          | Deliverable                                       | Path                                                  | Primary owner       | Done criteria                                                                       |
| --- | ----------------- | ------------------------------------------------- | ----------------------------------------------------- | ------------------- | ----------------------------------------------------------------------------------- |
| D1  | Source of truth   | Production readiness index (master index + links) | `docs/source-of-truth/README.md`                      | PM                  | Lists all canonical docs; each link resolves; includes last-updated date and owner. |
| D2  | Source of truth   | System overview and boundaries                    | `docs/source-of-truth/system-overview.md`             | Tech Lead           | Defines scope, architecture boundaries, critical dependencies, and non-goals.       |
| D3  | Source of truth   | Operational decision log                          | `docs/source-of-truth/decision-log.md`                | PM + Tech Lead      | Includes date-stamped decisions, rationale, impact, and owner for each decision.    |
| D4  | Source of truth   | Ownership and escalation matrix                   | `docs/source-of-truth/ownership-escalation-matrix.md` | PM                  | Every critical area has primary/secondary owner and escalation path.                |
| D5  | Operator runbook  | Service health triage runbook                     | `docs/runbooks/service-health-triage.md`              | Ops Lead            | Contains trigger, diagnosis flow, containment steps, and exit conditions.           |
| D6  | Operator runbook  | Rollback and recovery runbook                     | `docs/runbooks/rollback-and-recovery.md`              | Tech Lead           | Includes rollback triggers, rollback steps, data checks, and communication steps.   |
| D7  | Operator runbook  | Data backup and restore runbook                   | `docs/runbooks/backup-restore.md`                     | Data owner          | Restore drill completed and results documented; RTO/RPO assumptions stated.         |
| D8  | Operator runbook  | Security incident first response                  | `docs/runbooks/security-first-response.md`            | Security owner      | Defines severity levels, first-hour actions, evidence handling, and escalation.     |
| D9  | Onboarding/setup  | Contributor setup guide                           | `docs/onboarding/setup-guide.md`                      | Engineering         | Fresh-machine setup validated by one teammate not authoring the doc.                |
| D10 | Onboarding/setup  | Day-1 onboarding checklist                        | `docs/onboarding/day-1-checklist.md`                  | PM                  | New joiner can complete checklist without synchronous help except access approval.  |
| D11 | Onboarding/setup  | Troubleshooting and FAQ                           | `docs/onboarding/troubleshooting.md`                  | Engineering         | Top setup/release blockers documented with fix steps and owner to update.           |
| D12 | Release notes     | Release notes template                            | `docs/release/release-notes-template.md`              | PM                  | Template includes what changed, risk, migration/config impact, rollback, and owner. |
| D13 | Release notes     | Production release notes (current release)        | `docs/release/releases/<YYYY-MM-DD>-production.md`    | PM + Tech Lead      | Fully populated using template; reviewed by engineering and ops.                    |
| D14 | PR alignment      | PR template alignment guide                       | `docs/release/pr-template-alignment.md`               | PM                  | Maps release and comms requirements to `.github/pull_request_template.md` sections. |
| D15 | PR alignment      | PR author quality checklist                       | `docs/release/pr-author-checklist.md`                 | Engineering Manager | Includes testing/perf/risk expectations and points to `.codex/verify.commands`.     |
| D16 | Stakeholder comms | Stakeholder launch brief                          | `docs/comms/stakeholder-launch-brief.md`              | PM                  | Includes launch scope, value, user impact, risks, and support model.                |
| D17 | Stakeholder comms | Release announcement template                     | `docs/comms/release-announcement-template.md`         | PM/Comms            | Has internal + external variants and approved distribution channels.                |
| D18 | Stakeholder comms | Weekly status update template                     | `docs/comms/weekly-status-template.md`                | PM                  | Uses consistent sections: progress, risks, decisions, asks, next week.              |
| D19 | Stakeholder comms | Post-release closeout report                      | `docs/comms/post-release-closeout-report.md`          | PM                  | Contains outcomes vs goal, incidents, follow-ups, and final signoff record.         |

## 4.1) Current Artifact Status (2026-03-01)

Status key: Done | In Progress | Not Started

| ID  | Status | Notes                                                     |
| --- | ------ | --------------------------------------------------------- |
| D1  | Done   | Source-of-truth index exists and links canonical docs.    |
| D2  | Done   | System overview and boundaries documented.                |
| D3  | Done   | Decision log created and ready for ongoing updates.       |
| D4  | Done   | Ownership matrix available with escalation paths.         |
| D5  | Done   | Service health triage runbook present.                    |
| D6  | Done   | Rollback and recovery runbook present.                    |
| D7  | Done   | Backup and restore runbook created.                       |
| D8  | Done   | Security first-response runbook present.                  |
| D9  | Done   | Setup guide available.                                    |
| D10 | Done   | Day-1 onboarding checklist created.                       |
| D11 | Done   | Troubleshooting guide created.                            |
| D12 | Done   | Release notes template present.                           |
| D13 | Done   | Production release notes draft created for current cycle. |
| D14 | Done   | PR template alignment guide present.                      |
| D15 | Done   | PR author checklist present.                              |
| D16 | Done   | Stakeholder launch brief template present.                |
| D17 | Done   | Release announcement template created.                    |
| D18 | Done   | Weekly status template created.                           |
| D19 | Done   | Post-release closeout report template created.            |

## 5) Sequencing (Execution Order)

### Phase 0: Kickoff and baseline (Day 0)

1. Confirm owners for each deliverable (D1-D19).
2. Confirm release target date and communication audiences.
3. Freeze document folder structure and naming conventions.

Exit criteria:

- Owners assigned for every deliverable.
- Release date and audience list approved.

### Phase 1: Source-of-truth foundation (Days 1-2)

1. Deliver D1-D4.
2. Review for contradictions with README, PR template, and current release process.
3. Resolve all conflicting guidance before moving forward.

Exit criteria:

- D1-D4 approved by PM + Tech Lead.
- No unresolved contradictions across canonical docs.

### Phase 2: Operator readiness (Days 2-4)

1. Deliver D5-D8.
2. Run one tabletop exercise for triage + rollback.
3. Capture gaps and patch runbooks within 24 hours.

Exit criteria:

- Tabletop completed with notes.
- Runbooks updated from exercise findings.

### Phase 3: Onboarding and release mechanics (Days 3-5)

1. Deliver D9-D15.
2. Validate setup guide with a new or rotating contributor.
3. Validate PR alignment against `.github/pull_request_template.md` and `.codex/verify.commands`.

Exit criteria:

- Fresh-machine onboarding validation passed.
- PR/release documentation alignment signed off by PM + Engineering Manager.

### Phase 4: Stakeholder communications package (Days 4-6)

1. Deliver D16-D18.
2. Tailor messaging for internal operators, leadership, and end users.
3. Approve communication send list and cadence.

Exit criteria:

- Stakeholder artifacts approved by PM + Sponsor.
- Distribution plan confirmed.

### Phase 5: Release-week closeout (Release day + 2 days)

1. Publish D13 using the approved template (D12).
2. Send release announcement using D17.
3. Publish D19 after stabilization review.

Exit criteria:

- Release notes published.
- Announcement sent.
- Closeout report and signoff completed.

## 6) Done Criteria (Workstream Completion Gate)

The closeout workstream is complete only when all conditions are true:

1. All deliverables D1-D19 exist at target paths and are approved by named owners.
2. Runbooks (D5-D8) have been exercised at least once and updated from findings.
3. Onboarding setup (D9-D11) is validated by a non-author.
4. PR and release alignment artifacts (D12-D15) match current repository templates and verification commands.
5. Stakeholder communication artifacts (D16-D19) are published/sent per plan.
6. No open P0/P1 documentation blockers remain.

## 7) Governance and Cadence

- Working cadence: 20-minute daily closeout standup during phases 1-5.
- Risk review: End-of-day risk log update by PM.
- Change control: Any late change to release messaging or runbooks requires PM + Tech Lead approval.

## 8) Risks and Mitigations

- Risk: Docs drift from actual release behavior.
  - Mitigation: Validate D14/D15 directly against `.github/pull_request_template.md` and `.codex/verify.commands` before signoff.
- Risk: Runbooks are documented but untested.
  - Mitigation: Tabletop exercise is mandatory gate in phase 2.
- Risk: Stakeholder messages become inconsistent across channels.
  - Mitigation: D17 template becomes mandatory source for all release announcements.
- Risk: Ownership gaps after launch.
  - Mitigation: D4 ownership matrix required before phase 2 starts.

## 9) Suggested Tracking Board Columns

- Backlog
- In Progress
- In Review
- Approved
- Published

Use deliverable IDs (D1-D19) as ticket keys for weekly tracking.
