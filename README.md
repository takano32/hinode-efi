# hinode-efi

A generic Rust-based UEFI application for AArch64 systems.

`hinode-efi` boots on AArch64 UEFI firmware, prints system information
through UEFI console services, and returns control to the firmware.
It is written in Rust with `#![no_std]` and `#![no_main]`.

The name **Hinode** (日の出) means "sunrise" in Japanese.

## What it does

On each boot, `hinode-efi` outputs:

```text
hinode-efi
A generic Rust-based UEFI project for AArch64 systems.
Target: aarch64-unknown-uefi

Firmware vendor:   Debian distribution of EDK II
Firmware revision: 0x00010000
UEFI revision:     2.7

Memory map entries: 80
Total memory:       576 MiB (147457 pages)
```

This has been verified by building and running under QEMU TCG emulation on
an AArch64 host.

## Scope

The project is intentionally generic. It avoids assumptions that tie it to
any particular board, vendor, or Linux distribution. Asahi Linux may serve
as a useful reference environment, but it is not a requirement.

## Prerequisites

- Rust (stable toolchain)
- `aarch64-unknown-uefi` target

```sh
rustup target add aarch64-unknown-uefi
rustup component add rustfmt clippy
```

## Build

```sh
# debug
cargo build --target aarch64-unknown-uefi

# release
cargo build --release --target aarch64-unknown-uefi
```

Output binaries:

```text
target/aarch64-unknown-uefi/debug/hinode.efi
target/aarch64-unknown-uefi/release/hinode.efi
```

## Code quality

```sh
cargo fmt --all -- --check
cargo clippy --target aarch64-unknown-uefi -- -D warnings
```

## Run with QEMU

### Prerequisites

**Ubuntu / Debian:**

```sh
sudo apt install qemu-system-arm qemu-efi-aarch64
```

**Arch Linux:**

`edk2-aarch64` is not in the official repositories. Extract the firmware
from the Debian package instead:

```sh
mkdir -p /tmp/edk2-fw && cd /tmp/edk2-fw
curl -sL "https://ftp.debian.org/debian/pool/main/e/edk2/qemu-efi-aarch64_2025.02-9_all.deb" \
  -o qemu-efi-aarch64.deb
ar x qemu-efi-aarch64.deb
bsdtar -xf data.tar.xz
# firmware: ./usr/share/qemu-efi-aarch64/QEMU_EFI.fd
```

Then set the environment variable:

```sh
export QEMU_EFI=/tmp/edk2-fw/usr/share/qemu-efi-aarch64/QEMU_EFI.fd
```

Alternatively, build from the AUR (takes longer):

```sh
paru -S edk2-armvirt-git
```

### Manual run

```sh
./scripts/run-qemu-aarch64.sh --release
```

The script builds the release binary, copies it to `esp/EFI/BOOT/BOOTAA64.EFI`,
and launches QEMU. Serial output appears in the terminal.

On AArch64 hosts with `/dev/kvm`, KVM acceleration is used automatically.
On x86_64 hosts or when KVM is unavailable, QEMU TCG emulation is used.

### Smoke test

```sh
./scripts/qemu-smoke.sh
```

Runs QEMU with a 20-second timeout and exits 0 if `hinode-efi` appears in
the serial output. This is the same check run by CI.

Example output:

```text
UEFI firmware (version 2025.02-9 built at 20:16:19 on Sep  1 2025)
BdsDxe: loading Boot0002 "UEFI Misc Device 2" ...
hinode-efi
A generic Rust-based UEFI project for AArch64 systems.
Target: aarch64-unknown-uefi

Firmware vendor:   Debian distribution of EDK II
Firmware revision: 0x00010000
UEFI revision:     2.7

Memory map entries: 80
Total memory:       576 MiB (147457 pages)
[ INFO]:  src/main.rs@038: hinode-efi: boot complete
```

## Deploying to real hardware

Copy the release binary to the EFI system partition:

```sh
cp target/aarch64-unknown-uefi/release/hinode.efi \
   /boot/efi/EFI/BOOT/BOOTAA64.EFI
```

The application will run once at boot and return to the UEFI shell or
firmware menu.

## Continuous integration

One workflow: `.github/workflows/ci.yml`

Triggers: push to `main`/`master`, pull request, manual dispatch.

| Step | Command |
|------|---------|
| Formatting check | `cargo fmt --all -- --check` |
| Clippy | `cargo clippy --target aarch64-unknown-uefi -- -D warnings` |
| Debug build | `cargo build --target aarch64-unknown-uefi` |
| Release build | `cargo build --release --target aarch64-unknown-uefi` |
| Smoke test | `bash ./scripts/qemu-smoke.sh` |

CI runs on `ubuntu-24.04` with `qemu-system-arm` and `qemu-efi-aarch64`
installed via apt.

## Repository layout

```text
.
├── .agents/                  # Agent/AI collaboration instructions
│   ├── AGENTS.md             # Canonical agent instruction body
│   ├── SKILL.md              # Shared skill index
│   ├── agent-collaboration/
│   ├── hinode-efi/
│   ├── qemu-ci/
│   └── rust-uefi/
├── .github/
│   ├── dependabot.yml
│   └── workflows/
│       └── ci.yml
├── .gitignore
├── AGENTS.md                 # Import shim → .agents/AGENTS.md
├── CLAUDE.md                 # Import shim → .agents/AGENTS.md
├── Cargo.lock
├── Cargo.toml
├── README.md
├── SKILL.md                  # Import shim → .agents/SKILL.md
├── docs/
│   ├── asahi.md              # Relationship to Asahi Linux
│   ├── design.md             # Design notes and milestones
│   └── qemu.md               # QEMU setup and firmware notes
├── scripts/
│   ├── qemu-smoke.sh         # CI smoke test script
│   └── run-qemu-aarch64.sh   # Manual QEMU launch script
└── src/
    └── main.rs               # UEFI entry point
```

## Source overview

`src/main.rs` is the entire application:

```rust
#![no_main]
#![no_std]

use log::info;
use uefi::boot;
use uefi::mem::memory_map::{MemoryMap, MemoryType};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    // print banner, firmware info, memory map
    Status::SUCCESS
}
```

Key points:

- `#![no_std]` — no Rust standard library
- `#![no_main]` — entry point provided by `uefi::entry` macro
- `uefi::helpers::init()` — sets up allocator, logger, and panic handler
- `uefi::system::*` — reads firmware vendor and UEFI revision
- `uefi::boot::memory_map()` — reads the UEFI memory map
- Returns `Status::SUCCESS` — hands control back to firmware

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `uefi` | 0.37 | UEFI services, types, and macros |
| `log` | 0.4 | Logging facade (backed by uefi logger) |

## Design principles

**Generic first** — no board-specific or distro-specific assumptions in
the core code.

**Small core** — the entry point stays compact and readable. Abstractions
are added only when the minimal path is proven.

**Explicit boot-time behavior** — what happens at boot is visible in the
source, not hidden behind layers.

**Verified, not assumed** — firmware behavior is checked at runtime, not
taken for granted.

## Relationship to Asahi Linux

The name `hinode` was inspired by `asahi` (朝日, "morning sun"). This
project is not part of Asahi Linux and is not Apple Silicon-only.
The intended scope is generic AArch64 UEFI.

## Non-goals

- Full operating system
- Full firmware replacement
- Production secure boot solution
- Lock-in to a specific AArch64 board or vendor

## License

TBD.
