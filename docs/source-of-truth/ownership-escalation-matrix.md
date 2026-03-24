# Ownership and Escalation Matrix

## Functional Ownership

- Product scope and prioritization: PM
- Frontend implementation and UX stability: Frontend lane
- Rust/Tauri command and data integrity: Backend lane
- Quality gates and regression confidence: QA lane
- Release readiness and rollback posture: Release lane
- Operational docs and stakeholder updates: Docs/comms lane

## Escalation Path

1. Lane owner triage (same day)
2. Cross-lane sync for blockers impacting critical path
3. PM + release decision if blocker threatens go-live
4. Hold release when go/no-go gate is red

## Mandatory Escalation Triggers

- Any unresolved P0/P1 defect
- Failed canonical verify command in release candidate phase
- Missing rollback path for planned deployment
- Missing owner for a production incident domain
