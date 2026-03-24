# Execution Control Plane Contract

Last updated: 2026-03-01

## Objective

Define how autonomous execution proceeds safely from planning to release without ambiguity.

## Phase Progression Rule

A phase is complete only when all of its done criteria are met and the required quality gates are green.

## Lane Contract

Each lane must provide:

- Input: required context and dependencies
- Output: concrete artifact or merged code change
- Done criteria: objective checks that can pass/fail
- Escalation: when to stop and notify owner

## Required Lanes

- Planner lane: sequencing and acceptance criteria lock
- Builder lane: code implementation
- QA lane: verify + test evidence
- Release lane: deployment and rollback posture
- Docs/Comms lane: source-of-truth updates and release narrative

## Safety Posture

- Default execution uses least privilege and non-destructive operations.
- Experimental orchestration features are optional helpers, not critical-path requirements.
- No phase advances on red checks.

## Quality Gates (Fail-Closed)

- Frontend type/build check
- Rust fmt/lint/test check
- Performance guard set from `.codex/verify.commands`

## Handoff Rule

Every lane handoff must include:

1. What changed
2. Verification status
3. Remaining risks
4. Next owner
