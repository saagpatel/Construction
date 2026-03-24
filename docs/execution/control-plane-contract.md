# Control Plane Contract

## Objective

Ensure autonomous execution remains safe, convergent, and release-focused.

## Required Controls

- Orchestration backbone: `codex-execution-os`
- Canonical verification contract: `.codex/verify.commands`
- Deterministic execution entrypoint: `.codex/scripts/run_verify_commands.sh`
- Critical path tasks must not depend on experimental capability toggles.

## Lane Handoff Contract

Each lane handoff must include:

- Input: source artifact(s), assumptions, dependency context
- Output: concrete changes and verification evidence
- Done criteria: objective pass/fail checks
- Escalation rule: smallest unblock action when blocked

## Quality Progression Rule

- No phase progression on red gates.
- If a gate fails, remediation is prioritized over new scope.

## Merge Discipline

- Keep changes atomic by concern.
- Converge through canonical verify pass before release candidate freeze.
