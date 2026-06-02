#!/usr/bin/env bash
set -euo pipefail

TARGET="${RUST_TARGET:-aarch64-unknown-uefi}"
MODE="debug"

if [[ "${1:-}" == "--release" ]]; then
  MODE="release"
fi

if [[ "$MODE" == "release" ]]; then
  cargo build --release --target "$TARGET"
else
  cargo build --target "$TARGET"
fi

EFI_BIN="target/${TARGET}/${MODE}/hinode.efi"
ESP_DIR="${ESP_DIR:-esp}"
ESP_IMG="${ESP_IMG:-${ESP_DIR}.img}"
QEMU="${QEMU:-qemu-system-aarch64}"

if ! command -v "$QEMU" >/dev/null 2>&1; then
  echo "error: qemu-system-aarch64 was not found." >&2
  echo "Ubuntu: sudo apt install qemu-system-arm qemu-efi-aarch64" >&2
  echo "Arch:   sudo pacman -S qemu-system-aarch64 edk2-aarch64" >&2
  exit 1
fi

if [[ ! -f "$EFI_BIN" ]]; then
  echo "error: EFI binary not found: $EFI_BIN" >&2
  exit 1
fi

find_firmware() {
  if [[ -n "${QEMU_EFI:-}" ]]; then
    printf '%s\n' "$QEMU_EFI"
    return
  fi

  for candidate in \
    /usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
    /usr/share/edk2/aarch64/QEMU_EFI.fd \
    /usr/share/AAVMF/AAVMF_CODE.fd \
    /usr/share/AAVMF/AAVMF_CODE.ms.fd \
    /tmp/edk2-fw/usr/share/qemu-efi-aarch64/QEMU_EFI.fd
  do
    if [[ -f "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return
    fi
  done

  return 1
}

FIRMWARE="$(find_firmware || true)"
if [[ -z "$FIRMWARE" ]]; then
  echo "error: AArch64 UEFI firmware was not found." >&2
  echo "Ubuntu: sudo apt install qemu-system-arm qemu-efi-aarch64" >&2
  echo "Arch:   sudo pacman -S qemu-system-aarch64 edk2-aarch64" >&2
  echo "Or set QEMU_EFI=/path/to/QEMU_EFI.fd" >&2
  exit 1
fi

mkdir -p "${ESP_DIR}/EFI/BOOT"
cp "$EFI_BIN" "${ESP_DIR}/EFI/BOOT/BOOTAA64.EFI"
cp "$EFI_BIN" "${ESP_DIR}/hinode.efi"

cat > "${ESP_DIR}/startup.nsh" <<'EOF'
fs0:\EFI\BOOT\BOOTAA64.EFI
EOF

for tool in truncate mkfs.vfat mmd mcopy; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "error: $tool was not found." >&2
    echo "Ubuntu: sudo apt install dosfstools mtools" >&2
    echo "Arch:   sudo pacman -S dosfstools mtools" >&2
    exit 1
  fi
done

rm -f "$ESP_IMG"
truncate -s "${ESP_IMAGE_SIZE:-64M}" "$ESP_IMG"
mkfs.vfat -F 32 "$ESP_IMG" >/dev/null
mmd -i "$ESP_IMG" ::/EFI ::/EFI/BOOT
mcopy -i "$ESP_IMG" "$EFI_BIN" ::/EFI/BOOT/BOOTAA64.EFI
mcopy -i "$ESP_IMG" "$EFI_BIN" ::/hinode.efi
mcopy -i "$ESP_IMG" "${ESP_DIR}/startup.nsh" ::/startup.nsh

echo "EFI binary: ${EFI_BIN}"
echo "ESP dir:    ${ESP_DIR}"
echo "ESP image:  ${ESP_IMG}"
echo "Firmware:   ${FIRMWARE}"
echo "QEMU:       ${QEMU}"

machine_args=(-machine virt)
cpu_args=(-cpu cortex-a57)

if [[ "$(uname -m)" =~ ^(aarch64|arm64)$ && -e /dev/kvm ]]; then
  machine_args=(-machine virt,accel=kvm)
  cpu_args=(-cpu host)
  echo "KVM:        enabled"
else
  echo "KVM:        not used; using TCG emulation"
fi

exec "$QEMU" \
  "${machine_args[@]}" \
  "${cpu_args[@]}" \
  -m "${QEMU_MEMORY:-512M}" \
  -nographic \
  -net none \
  -bios "$FIRMWARE" \
  -drive "if=none,format=raw,file=${ESP_IMG},id=esp" \
  -device virtio-blk-device,drive=esp
