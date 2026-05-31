# hinode-efi skill

Path: `.agents/hinode-efi/SKILL.md`

Focused support skill for project identity and scope.

Canonical main file: `.agents/AGENTS.md`.

## Identity

`hinode-efi` is a generic Rust-based UEFI project for AArch64 systems.

It is not Asahi Linux-only and it is not Apple Silicon-only.

Keep these names consistent:

- `hinode-efi`
- `hinode`
- `hinode.efi`
- `aarch64-unknown-uefi`
- `EFI/BOOT/BOOTAA64.EFI`

Do not recreate `FILES.txt`, `MANIFEST.tsv`, `rust-toolchain.toml`, or
`.cargo/config.toml` unless the user explicitly asks.
