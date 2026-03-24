# Control Plane Contract

Last updated: 2026-03-01

## Objective

Define a safe, repeatable execution contract for autonomous delivery.

## Orchestration backbone

- Primary workflow: `codex-execution-os`
- Supporting workflows:
  - `quality-gatekeeper`
  - `docs-knowledge-hub`
  - `release-command`

## Lane model

- Planner lane: phase sequencing, acceptance criteria, dependency lock.
- Explorer lane: discovery and reality validation.
- Builder lane: code and configuration changes.
- QA lane: verification and regression checks.
- Release lane: release readiness and rollback posture.
- Docs/comms lane: source-of-truth and stakeholder artifacts.

## Typed handoff contract (required)

Every lane handoff must include:

1. Input context: problem statement + scope boundaries.
2. Output artifact: changed files or explicit no-change decision.
3. Done criteria: testable acceptance checks.
4. Open risks: known unknowns and blocking assumptions.
5. Escalation trigger: explicit condition that requires release-lane pause.

## Gate policy

- Fail-closed on required gates.
- No phase progression when a required gate is red.
- Canonical verification source is `.codex/verify.commands`.

## Safety policy

- Use smallest possible change slices.
- Prefer stable tooling paths; treat experimental orchestration as optional.
- Do not proceed to ship without rollback steps documented in `docs/runbooks/rollback-and-recovery.md`.
