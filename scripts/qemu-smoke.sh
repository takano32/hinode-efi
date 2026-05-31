#!/usr/bin/env bash
set -euo pipefail

LOG="${QEMU_SMOKE_LOG:-qemu-smoke.log}"
TIMEOUT="${QEMU_SMOKE_TIMEOUT:-20s}"

rm -f "$LOG"

set +e
timeout "$TIMEOUT" bash ./scripts/run-qemu-aarch64.sh --release >"$LOG" 2>&1
status=$?
set -e

cat "$LOG"

# QEMU usually keeps running after the EFI app returns to firmware.
# A timeout is acceptable if the expected serial output appeared first.
if grep -q "hinode-efi" "$LOG"; then
  echo "qemu-smoke: saw hinode-efi output"
  exit 0
fi

echo "qemu-smoke: hinode-efi output was not found" >&2
echo "qemu-smoke: qemu exit status was $status" >&2
exit 1
