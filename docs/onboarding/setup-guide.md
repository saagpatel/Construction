# Setup Guide

## Prerequisites

- Node.js 20+
- pnpm
- Rust toolchain (stable)
- Tauri build dependencies for your OS

## First-Time Setup

1. `pnpm install`
2. `cd src-tauri && cargo build`
3. `pnpm dev:normal`

## Verification Setup

Run the canonical quality gate:

```bash
bash .codex/scripts/run_verify_commands.sh
```

## Common Commands

- `pnpm dev:normal` for standard Tauri development
- `pnpm dev:lean` for lower-disk development mode
- `pnpm tauri build` for production package artifacts
