# Construction Safety Tracker

[![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> OSHA compliance without the spreadsheet archaeology

Construction Safety Tracker is a desktop application for managing workplace safety incidents, running root cause analysis, generating OSHA compliance reports, and tracking corrective actions — built for construction firms to turn regulatory compliance into a workflow rather than a quarterly scramble.

## Features

- **Incident management** — full CRUD with multi-step create wizard, employee info, injury/illness classification, and auto-assigned case numbers per establishment per year
- **OSHA 300 / 300A / 301** — auto-generated compliance forms with CSV export; TRIR calculation and annual statistics management
- **Root cause analysis** — 5-Why, fishbone, and checklist-driven analysis workflows tied to each incident record
- **Corrective action tracking** — assigned actions with due dates, status tracking, and completion verification
- **Attachment support** — photos, audio, and document attachments stored locally via Tauri FS plugin
- **Privacy case handling** — automatic name masking for privacy-designated incidents per OSHA requirements

## Quick Start

### Prerequisites
- Node.js 18+
- Rust 1.75+ and Cargo
- Tauri CLI: `cargo install tauri-cli`

### Installation
```bash
pnpm install
```

### Usage
```bash
# Development
pnpm run dev

# Production build
pnpm run build

# Run Rust tests
pnpm run test:rust
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Shell | Tauri 2 (Rust backend) |
| Language | TypeScript + React |
| Routing | React Router |
| Charts | Recharts |
| State | Zustand |
| Bundler | Vite |

## Architecture

The Tauri Rust backend owns all file I/O and business logic validation — incident creation, OSHA form generation, and CSV export are Tauri commands that the React frontend invokes via `invoke()`. SQLite (via `tauri-plugin-sql`) stores all incident records locally with no cloud dependency. The OSHA form renderer reads raw incident data and applies the federal form calculation rules in Rust, keeping compliance logic auditable and separate from the UI layer.

## License

MIT