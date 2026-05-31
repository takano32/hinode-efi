# Design notes

`hinode-efi` starts as a minimal Rust UEFI application for AArch64 systems.

## Core idea

The project should keep a small generic core and add platform-specific behavior
only when it is explicitly named and isolated.

## Initial milestones

1. ✓ Build a minimal `hinode.efi` binary.
2. ✓ Print basic output through UEFI console services.
3. ✓ Inspect basic firmware/system information.
4. ✓ Inspect the UEFI memory map.
5. Experiment with loading external payloads.
6. Keep AArch64 as the primary target while avoiding device-specific lock-in.

## Principles

- Prefer readable code over clever code.
- Keep boot-time behavior explicit.
- Avoid hidden board/vendor assumptions.
- Treat firmware behavior as something to verify, not something to assume.
