# .agents/AGENTS.md

Canonical agent instructions for `hinode-efi`.

This file is the main agent instruction body. Root-level `AGENTS.md`,
`CLAUDE.md`, and `SKILL.md` are only import shims that point into `.agents/`.

## Imports

Read the shared skill index after this file:

@.agents/SKILL.md

The skill index imports the focused skills:

```text
.agents/hinode-efi/SKILL.md
.agents/rust-uefi/SKILL.md
.agents/qemu-ci/SKILL.md
.agents/agent-collaboration/SKILL.md
```

## Project identity

`hinode-efi` is a generic Rust-based UEFI project for AArch64 systems.

It is not Asahi Linux-only and it is not Apple Silicon-only. The name was
inspired by the Asahi ecosystem, but the project scope is generic AArch64 UEFI.

Keep these names consistent unless the user explicitly requests a rename:

- Project name: `hinode-efi`
- Cargo package: `hinode-efi`
- Binary target: `hinode`
- Expected EFI binary: `hinode.efi`
- Rust target: `aarch64-unknown-uefi`
- Default removable-media path: `EFI/BOOT/BOOTAA64.EFI`

## Repository shape

The intended practical repository shape is:

```text
.
├── .agents/
├── .github/
├── .gitignore
├── AGENTS.md
├── CLAUDE.md
├── Cargo.toml
├── README.md
├── SKILL.md
├── docs/
├── scripts/
└── src/
```

Root-level `AGENTS.md`, `CLAUDE.md`, and `SKILL.md` must stay small import shims.
Root `SKILL.md` imports `.agents/SKILL.md`; `.agents/SKILL.md` imports the split
skill files.

Do not recreate these unless the user explicitly asks:

- `FILES.txt`
- `MANIFEST.tsv`
- `rust-toolchain.toml`
- `.cargo/config.toml`
- extra workflow files such as `.github/workflows/qemu-smoke.yml`

## Scope rules

The core project should stay generic AArch64 UEFI.

Allowed at this stage:

- minimal Rust UEFI application
- AArch64 UEFI console output
- simple project documentation
- build-only CI checks
- QEMU smoke test inside the single `ci.yml` workflow
- helper scripts required by CI

Avoid at this stage:

- making Asahi Linux a hard requirement
- making Apple Silicon a hard requirement
- hiding board-specific assumptions in generic code
- adding complex abstractions before the minimal boot path is proven
- adding overlapping GitHub workflow files

## Rust and Cargo rules

`Cargo.toml` is required.

Use explicit target commands. Do not rely on `.cargo/config.toml`.

```sh
rustup target add aarch64-unknown-uefi
cargo fmt --all -- --check
cargo clippy --target aarch64-unknown-uefi -- -D warnings
cargo build --target aarch64-unknown-uefi
cargo build --release --target aarch64-unknown-uefi
```

Expected outputs:

```text
target/aarch64-unknown-uefi/debug/hinode.efi
target/aarch64-unknown-uefi/release/hinode.efi
```

Coding rules:

- Keep the UEFI entry point small.
- Prefer explicit, readable boot-time code.
- Avoid adding abstractions until the minimal build path is proven.
- Keep `#![no_std]` and `#![no_main]` unless there is a deliberate reason to
  change them.
- Do not add platform-specific assumptions to `src/main.rs`.
- Keep output strings stable if CI/QEMU smoke tests depend on them.
- If the smoke test greps for `hinode-efi`, do not remove that output without
  updating the test.

## QEMU and KVM rules

QEMU helper scripts are kept because the CI QEMU smoke job uses them.

Local smoke test:

```sh
./scripts/qemu-smoke.sh
```

Manual release run:

```sh
./scripts/run-qemu-aarch64.sh --release
```

Expected smoke-test behavior:

1. Build the release EFI binary.
2. Copy it to `esp/EFI/BOOT/BOOTAA64.EFI`.
3. Launch AArch64 UEFI firmware with QEMU.
4. Capture serial output.
5. Pass only if output contains `hinode-efi`.

KVM / TCG rule:

- On AArch64 hosts with `/dev/kvm`, KVM may be used.
- On x86_64 hosts, QEMU TCG emulation is expected.
- Do not describe TCG as KVM.
- Do not claim KVM was used unless the log shows KVM was enabled.

## GitHub Actions rules

There should be one main workflow:

```text
.github/workflows/ci.yml
```

It should run on:

- push to `main` or `master`
- pull request
- manual `workflow_dispatch`

Expected checks:

1. formatting check
2. Clippy
3. debug build
4. release build
5. QEMU smoke test using `scripts/qemu-smoke.sh`

Keep GitHub Actions simple:

- Prefer simple YAML.
- Prefer official or broadly standard actions.
- Do not add caches until slow CI has been observed.
- Do not add matrix builds until there is more than one supported target.
- Keep `RUST_TARGET` set to `aarch64-unknown-uefi`.
- Keep QEMU smoke testing inside `.github/workflows/ci.yml`.
- Do not add `.github/workflows/qemu-smoke.yml`.

## Claude and Codex collaboration

Codex entry point:

```text
AGENTS.md
```

Claude Code entry point:

```text
CLAUDE.md
```

Both root files should import or point to:

```text
.agents/AGENTS.md
```

When handing work to another agent, include:

- what changed
- why it changed
- what was not run
- what still needs verification
- any files that should not be renamed or regenerated

If instructions conflict, use this priority:

1. User request in the current task
2. `.agents/AGENTS.md`
3. Relevant `.agents/*/SKILL.md`
4. Root `AGENTS.md` / `CLAUDE.md` / `SKILL.md` import shims
5. `README.md` and docs

## Verification language

Be precise about verification.

Use:

```text
Checked: YAML parse
Not run: cargo build, QEMU
```

Do not claim a build, QEMU run, KVM run, or GitHub Actions run succeeded unless
it was actually executed.

## Current caveat

The repository files are intended to be internally consistent, but this generated
project has not yet been proven by an actual `cargo build` or QEMU run in this
environment.

Do not remove this caveat until those commands have actually succeeded.
