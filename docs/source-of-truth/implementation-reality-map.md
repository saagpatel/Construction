# Implementation Reality Map

Last updated: 2026-03-01

## Core capabilities implemented

- Establishments and locations CRUD
- Incident CRUD (create/read/update/delete), filtering, and attachments
- RCA workflows: 5 Whys, fishbone categories/causes, corrective actions
- OSHA reporting:
  - OSHA 300 log
  - OSHA 300A summary
  - OSHA 301 report
  - CSV exports
- Dashboard aggregates for incidents and corrective actions
- CSV incident import preview and import
- Toolbox topics/talks/attendees/signatures
- JSA templates, instances, steps, and status transitions (draft -> reviewed -> approved -> in progress -> completed)

## Capabilities partially implemented

- Security and auth foundations exist in schema but are not enforced end-to-end in command layer.
- Offline and sync data tables exist, but sync execution is not wired through a runtime service.
- Performance checks exist, but API/DB perf jobs depend on environment variables and are optional in CI.

## Planned in schema but not yet implemented in app layer

- Safety inspections
- Near miss reporting
- Training records
- Equipment tracking
- Trade hazard library APIs and UI workflows

## Quality and release controls now in place

- Canonical verification commands include:
  - git hygiene
  - frontend typecheck/build
  - Rust lint + tests
  - performance checks (bundle/build/assets/memory)
- CI quality checks are split across workflows:
  - `quality-gates.yml` for frontend and Rust quality checks
  - `git-hygiene.yml` and `lockfile-rationale.yml` for contribution controls
  - `perf-enforced.yml` for production-profile performance enforcement

## Decision

To avoid scope drift, production completion should prioritize:

1. Reliability and release safety of implemented workflows.
2. Honest documentation of non-implemented modules.
3. Explicit backlog tracking for schema-only modules.
