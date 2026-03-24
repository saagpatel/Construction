# Onboarding Troubleshooting

## `pnpm install` fails

- Ensure Node version matches project minimum.
- Clear lockfile cache: `pnpm store prune`.
- Retry with `pnpm install --frozen-lockfile`.

## Rust build fails

- Confirm stable toolchain: `rustup show`.
- Install missing target dependencies for your OS.
- Retry: `cd src-tauri && cargo build`.

## Tauri app will not start

- Run `pnpm check:frontend` first to verify web build health.
- Ensure no other process is holding app dev ports.
- Retry with `pnpm dev:normal`.

## Verify command fails

- Run the failing command from `.codex/verify.commands` directly.
- Fix failures in order; do not skip earlier failing gates.
- Re-run `bash .codex/scripts/run_verify_commands.sh`.

## CI passes locally but fails in PR

- Confirm branch naming and commit format requirements.
- Check PR body has required sections (lockfile/baseline governance when applicable).
- Re-run local verify after syncing latest branch.

## Escalation

- Build failures: engineering owner
- Release gate failures: release owner
- Security failures: security owner
