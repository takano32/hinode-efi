# hinode-efi

A generic Rust-based UEFI project for AArch64 systems.

`hinode-efi` is an experimental UEFI project written in Rust. It targets
AArch64 UEFI environments and starts from a small, readable EFI application.

The name **Hinode** means "sunrise" in Japanese: the quiet moment before the
system wakes.

## Scope

`hinode-efi` is designed as a generic AArch64 UEFI project.

Although the name was inspired by the Asahi ecosystem, this project is **not**
limited to Asahi Linux or Apple Silicon. Asahi Linux may be one useful test
environment, but the goal is to support AArch64 UEFI systems more broadly.

## Current status

The initial boot path is proven.

At startup, `hinode-efi` prints:

```text
hinode-efi
A generic Rust-based UEFI project for AArch64 systems.
Target: aarch64-unknown-uefi

Firmware vendor:   <vendor string>
Firmware revision: <hex>
UEFI revision:     <major.minor>

Memory map entries: <n>
Total memory:       <N> MiB (<pages> pages)
```

This has been verified with `cargo build`, `cargo clippy`, and a QEMU TCG
smoke test (`./scripts/qemu-smoke.sh`).

## Target

The primary Rust target is:

```text
aarch64-unknown-uefi
```

The expected EFI binary is:

```text
hinode.efi
```

For default AArch64 removable-media boot, copy or rename it to:

```text
EFI/BOOT/BOOTAA64.EFI
```

## Repository layout

```text
.
├── .agents
│   ├── AGENTS.md
│   ├── README.md
│   ├── SKILL.md
│   ├── agent-collaboration
│   │   └── SKILL.md
│   ├── hinode-efi
│   │   └── SKILL.md
│   ├── qemu-ci
│   │   └── SKILL.md
│   └── rust-uefi
│       └── SKILL.md
├── .github
│   ├── dependabot.yml
│   └── workflows
│       └── ci.yml
├── .gitignore
├── AGENTS.md
├── CLAUDE.md
├── Cargo.toml
├── README.md
├── SKILL.md
├── docs
│   ├── asahi.md
│   ├── design.md
│   └── qemu.md
├── scripts
│   ├── qemu-smoke.sh
│   └── run-qemu-aarch64.sh
└── src
    └── main.rs
```

## Build

Install the Rust target:

```sh
rustup target add aarch64-unknown-uefi
```

Build:

```sh
cargo build --target aarch64-unknown-uefi
```

Release build:

```sh
cargo build --release --target aarch64-unknown-uefi
```

Expected outputs:

```text
target/aarch64-unknown-uefi/debug/hinode.efi
target/aarch64-unknown-uefi/release/hinode.efi
```

## Run with QEMU

The QEMU helper scripts are kept because CI uses them for the smoke test.

```sh
./scripts/qemu-smoke.sh
```

See [docs/qemu.md](docs/qemu.md) for package names and firmware path notes.

## Agent guidance

All detailed agent guidance lives under:

```text
.agents/
```

Root-level files are only import shims:

```text
AGENTS.md
CLAUDE.md
SKILL.md
```

The canonical agent instruction body is:

```text
.agents/AGENTS.md
```

The shared skill index is:

```text
.agents/SKILL.md
```

Focused supporting skills are split under:

```text
.agents/hinode-efi/SKILL.md
.agents/rust-uefi/SKILL.md
.agents/qemu-ci/SKILL.md
.agents/agent-collaboration/SKILL.md
```

## GitHub Actions

There is one workflow:

```text
.github/workflows/ci.yml
```

It runs on:

- push to `main` or `master`
- pull request
- manual `workflow_dispatch`

The workflow has two jobs:

1. build the AArch64 UEFI binary
2. boot the release binary with QEMU and check that the output contains
   `hinode-efi`

## Design principles

### Generic first

The project should avoid assumptions that only apply to one device family or
Linux distribution.

### Small core

The initial code should stay compact and understandable.

### Explicit platform behavior

When platform-specific behavior is needed, it should be named clearly rather
than hidden behind generic-looking code.

## Relationship to Asahi Linux

The name `hinode` was inspired by the word `asahi`, meaning "morning sun" or
"rising sun".

However, `hinode-efi` is not an Asahi Linux project and is not intended to be
Apple Silicon-only. The broader target is generic AArch64 UEFI.

## Non-goals

At least for now, `hinode-efi` is not intended to be:

- a full operating system
- a full firmware implementation
- a replacement for UEFI firmware
- a production-ready secure boot solution
- tied to a single AArch64 board or vendor

## License

TBD.
