#!/usr/bin/env bash
# Re-run every spec file through oracle and port, in parallel, without
# rebuilding per spec. Prints one line per failure and a tally.
#
# Usage: scripts/verify-all-specs.sh [jobs]
set -uo pipefail
ROOT="${LEMANS_ROOT:-$HOME/Projects/lemans-dc}"
JOBS="${1:-8}"
cd "$ROOT"
python3 scripts/gen-registry.py >/dev/null
BIN=analysis/lemans24.bin
ORACLE=rust/sh4-oracle/target/release/sh4-oracle
VERIFY=rust/lemans-verify/target/release/lemans-verify
run() {
  spec="$1"
  addr="$(basename "$spec" | cut -d. -f1 | cut -d- -f1)"
  out="$("$VERIFY" "$BIN" "$addr" "$spec" --oracle "$ORACLE" 2>&1)"
  if ! grep -q '^ok ' <<<"$out"; then
    printf 'FAIL %s :: %s\n' "$spec" "$(grep -m1 -E '^FAIL|difference|!=|bad spec|no port|oracle' <<<"$out")"
  fi
}
export -f run
export BIN ORACLE VERIFY
find notes/specs -name '0x*.json' -print0 | xargs -0 -n1 -P "$JOBS" bash -c 'run "$0"' > /tmp/verify-all.log 2>&1
fails=$(wc -l < /tmp/verify-all.log)
total=$(find notes/specs -name '0x*.json' | wc -l)
echo "specs $total  failures $fails"
head -20 /tmp/verify-all.log
[ "$fails" -eq 0 ]
