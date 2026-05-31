# QEMU and CI skill

Path: `.agents/qemu-ci/SKILL.md`

Focused support skill for QEMU, KVM, and GitHub Actions.

Canonical main file: `.agents/AGENTS.md`.

## Rules

- Keep one main workflow at `.github/workflows/ci.yml`.
- Keep `RUST_TARGET=aarch64-unknown-uefi`.
- Keep QEMU smoke testing inside `ci.yml`.
- Keep `scripts/qemu-smoke.sh` and `scripts/run-qemu-aarch64.sh` because CI uses them.
- Do not add `.github/workflows/qemu-smoke.yml`.
- QEMU smoke should pass only when output contains `hinode-efi`.
- On x86_64, use QEMU TCG emulation.
- Do not claim KVM or QEMU success unless actually run.
