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

Install QEMU:

```sh
sudo pacman -S qemu-system-aarch64
```

`edk2-aarch64` is not in the official Arch repositories. The firmware can be
extracted from the Debian package:

```sh
mkdir -p /tmp/edk2-fw && cd /tmp/edk2-fw
curl -sL "https://ftp.debian.org/debian/pool/main/e/edk2/qemu-efi-aarch64_2025.02-9_all.deb" \
  -o qemu-efi-aarch64.deb
ar x qemu-efi-aarch64.deb
bsdtar -xf data.tar.xz
# firmware is at: ./usr/share/qemu-efi-aarch64/QEMU_EFI.fd
```

Then run with:

```sh
QEMU_EFI=/tmp/edk2-fw/usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
  ./scripts/run-qemu-aarch64.sh --release
```

Alternatively, build `edk2-armvirt-git` from the AUR (takes time):

```sh
paru -S edk2-armvirt-git
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
