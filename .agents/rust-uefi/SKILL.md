# Rust UEFI skill

Path: `.agents/rust-uefi/SKILL.md`

Focused support skill for Rust and UEFI implementation work.

Canonical main file: `.agents/AGENTS.md`.

## Commands

```sh
rustup target add aarch64-unknown-uefi
cargo fmt --all -- --check
cargo clippy --target aarch64-unknown-uefi -- -D warnings
cargo build --target aarch64-unknown-uefi
cargo build --release --target aarch64-unknown-uefi
```

Keep `#![no_std]`, `#![no_main]`, and the `hinode-efi` output string unless a
deliberate change also updates tests and documentation.
