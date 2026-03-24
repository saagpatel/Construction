# System Overview

## Product Goal

A desktop-first construction safety tracker for incident reporting, OSHA reporting, RCA workflows, and core safety program operations.

## Implemented Runtime Modules

- Establishments and locations management
- Incident CRUD and filtering
- Incident attachment workflow
- Dashboard analytics (summary + incident distributions)
- OSHA 300 / 300A / 301 generation and CSV export
- RCA workflows (5 Whys, fishbone, corrective actions)
- Toolbox talks with attendee signatures
- JSA template usage and instance workflow
- CSV import pipeline for incidents

## Data/Foundation Modules Present But Not Runtime-Exposed

- Inspections schema + seed data
- Near-miss schema
- Training schema + seed courses
- Equipment schema
- Sync/auth/audit schema scaffolding

## Core Boundaries

- Frontend: React + Zustand + Router
- Desktop shell and command boundary: Tauri 2 command handlers
- Domain/data: Rust services over SQLite
- Verification: `.codex/verify.commands` via `.codex/scripts/run_verify_commands.sh`

## Non-Goals For Current Release Candidate

- Cloud sync and multi-device conflict management
- Runtime RBAC/auth session enforcement
- End-user UI for inspections/training/equipment/near-miss modules
