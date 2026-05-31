# QEMU / KVM smoke test

`hinode-efi` can be tested as an AArch64 UEFI application with QEMU.

The helper scripts are kept because the GitHub Actions CI uses them for the
QEMU smoke test.

## Install prerequisites

### Ubuntu / Debian

```sh
sudo apt update
sudo apt install qemu-system-arm qemu-efi-aarch64
```

### Arch Linux

```sh
sudo pacman -S qemu-system-aarch64 edk2-aarch64
```

## Run manually

Build and run the debug EFI binary:

```sh
./scripts/run-qemu-aarch64.sh
```

Build and run the release EFI binary:

```sh
./scripts/run-qemu-aarch64.sh --release
```

The script creates a local `esp/` directory and copies the EFI application to:

```text
esp/EFI/BOOT/BOOTAA64.EFI
```

## Smoke test

For CI-like testing:

```sh
./scripts/qemu-smoke.sh
```

The smoke test runs QEMU with a timeout and checks the serial output for:

```text
hinode-efi
```

A timeout is acceptable if the expected output appeared first, because the EFI
application may return to firmware and leave QEMU running.
