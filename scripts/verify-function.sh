#!/usr/bin/env bash
# Verify one ported function against the SH-4 oracle and record the evidence.
#
# Usage:  verify-function.sh <address> [binary]
#   address  0x8c0…  the original function's entry
#   binary   defaults to analysis/lemans24.bin; override for fixtures
#
# Writes notes/verifications/<address>.json. A ledger entry without one of these
# files is a claim, not a result.
set -euo pipefail

ADDR="${1:?usage: verify-function.sh <0xaddress> [binary]}"
ROOT="$HOME/Projects/lemans-dc"
BIN="${2:-$ROOT/analysis/lemans24.bin}"
SPEC="$ROOT/notes/specs/$ADDR.json"
VERIFY="$ROOT/rust/lemans-verify"
ORACLE="$ROOT/rust/sh4-oracle/target/release/sh4-oracle"
OUT="$ROOT/notes/verifications"

[ -f "$SPEC" ] || { echo "no spec at $SPEC" >&2; exit 2; }
[ -f "$BIN" ] || { echo "no binary at $BIN" >&2; exit 2; }

mkdir -p "$OUT"

# Build once; incremental builds are cheap after the first.
( cd "$ROOT/rust/sh4-oracle" && cargo build --release -q )
( cd "$VERIFY" && cargo build --release -q )

ORACLE_JSON="$("$ORACLE" "$BIN" --spec "$SPEC")"
set +e
RESULT="$("$VERIFY/target/release/lemans-verify" "$BIN" "$ADDR" "$SPEC" --oracle "$ORACLE" 2>&1)"
STATUS=$?
set -e

COMMIT="$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo unknown)"
SHA="$(sha256sum "$BIN" | cut -d' ' -f1)"

python3 - "$OUT/$ADDR.json" "$ADDR" "$SPEC" "$BIN" "$SHA" "$COMMIT" "$STATUS" "$RESULT" "$ORACLE_JSON" <<'PY'
import json, sys, pathlib, datetime
path, addr, spec, binary, sha, commit, status, result, oracle_json = sys.argv[1:10]
oracle = json.loads(oracle_json)
doc = {
    "address": addr,
    "spec": spec.replace(str(pathlib.Path.home()), "~"),
    "binary": binary.replace(str(pathlib.Path.home()), "~"),
    "binary_sha256": sha,
    "commit": commit,
    "verified_at": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
    "result": "pass" if status == "0" else "fail",
    "oracle": {
        "stop": oracle.get("stop"),
        "steps": oracle.get("steps"),
    },
    "verifier_output": result.strip().splitlines(),
}
pathlib.Path(path).write_text(json.dumps(doc, indent=2) + "\n")
print(f"{'ok  ' if status == '0' else 'FAIL'} {addr} -> {path}")
PY

echo "$RESULT"
exit "$STATUS"
