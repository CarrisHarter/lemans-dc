#!/usr/bin/env bash
# Canonical re-verification for a list of ports. Regenerates the registry once,
# then runs the real verify script per address and summarizes.
#
# Usage: scripts/verify-ports.sh <address> [<address> ...]
set -uo pipefail
ROOT="${LEMANS_ROOT:-$HOME/Projects/lemans-dc}"
cd "$ROOT"
python3 scripts/gen-registry.py >/dev/null

pass=0
fail=0
failed=()
for addr in "$@"; do
  if out="$(./scripts/verify-function.sh "$addr" 2>&1)" && grep -q "^ok " <<<"$out"; then
    pass=$((pass + 1))
  else
    fail=$((fail + 1))
    failed+=("$addr")
    echo "FAIL $addr"
    grep -E "FAIL|difference|r[0-9]+:|T bit|writes:|oracle" <<<"$out" | head -6
  fi
done
echo "verified $pass, failed $fail"
[ "$fail" -eq 0 ] || { printf 'failed: %s\n' "${failed[*]}"; exit 1; }
