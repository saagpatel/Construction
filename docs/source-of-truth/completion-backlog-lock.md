# Completion Backlog Lock

Last updated: 2026-03-01
Status: Active lock (post-hardening refresh)

## Objective

Define the exact, execution-ready backlog from current state to production-ready completion.

## Locked Backlog (Priority Order)

1. Quality gate enforcement

- Ensure frontend type/build checks and Rust fmt/lint/test checks are enforced locally and in CI.
- Acceptance: `pnpm verify` passes and PR `quality-gates` workflow passes.
- Status: Done

2. Incident evidence flow completion

- Ensure incident detail supports upload and management of photo/audio/document attachments in the primary flow.
- Acceptance: user can upload, view grouped attachment lists, and delete attachments from incident detail.
- Status: Done

3. Toolbox talk signature flow completion

- Add signature capture flow for attendees and persist signatures to backend.
- Acceptance: unsigned attendee can be signed in UI and status changes to signed after refresh.
- Status: Done

4. Docs and operational hardening

- Publish source-of-truth docs and runbooks for triage/rollback.
- Acceptance: docs exist at target paths and align with current code reality.
- Status: Done

5. Release gate readiness package

- Maintain release-readiness checklist and update release note artifact for current cycle.
- Acceptance: checklist completed with no open P0/P1 blockers.
- Status: In progress

## Remaining Pre-Ship Items

- Incident edit route (`/incidents/:id/edit`) for full frontend CRUD parity. - Done
- OSHA tab/label parity for 301 incident-level reporting UX. - Done
- JSA/RCA closeout UX completeness and clearer workflow handoff. - Done
- Release target/channel decision and credentials for publish.

## Change Control

Any new scope after this lock must be tagged as post-ship unless it blocks safety, quality, or release.
