#!/usr/bin/env bash
# Merge one workspace's fixes for the addresses it owned, then re-verify them.
#
# Usage: scripts/merge-fixes.sh <workspace> <address>...
#
# The agent that owns an address is the only writer of its port file, so the
# workspace copy wins over the canonical one. Specs are copied only when the
# name is new: an existing witness with different content is a conflict worth
# seeing, not a silent overwrite.
set -uo pipefail

WS="${1:?usage: merge-fixes.sh <workspace> <address>...}"
shift
ROOT="${LEMANS_ROOT:-$HOME/Projects/lemans-dc}"
cd "$ROOT"

# The workspace copy may predate the schema normalization; normalize before
# comparing so an equivalent spec is not reported as a conflict.
python3 scripts/normalize-specs.py $(find "$WS/notes/specs" -name '0x*.json') >/dev/null 2>&1 || true

fail=0
for addr in "$@"; do
  a="${addr#0x}"
  src="$WS/rust/lemans-verify/src/ports/p_$a.rs"
  dst="rust/lemans-verify/src/ports/p_$a.rs"
  if [ ! -f "$src" ]; then echo "MISSING port $src"; fail=1; continue; fi
  if [ -f "$dst" ] && cmp -s "$src" "$dst"; then
    :
  else
    cp "$src" "$dst" && echo "updated p_$a.rs"
  fi
  # Some agents write only tagged specs; the harness and the ledger require a
  # canonical <addr>.json, so promote the first tagged one if it is missing.
  if [ ! -f "notes/specs/0x$a.json" ] && [ ! -f "$WS/notes/specs/0x$a.json" ]; then
    first="$(ls "$WS"/notes/specs/0x$a.*.json 2>/dev/null | head -1 || true)"
    [ -n "$first" ] && cp "$first" "notes/specs/0x$a.json" && echo "promoted $(basename "$first") -> 0x$a.json"
  fi
  for spec in "$WS/notes/specs/0x$a.json" "$WS"/notes/specs/0x$a.*.json; do
    [ -f "$spec" ] || continue
    name="$(basename "$spec")"
    if [ -f "notes/specs/$name" ] && ! cmp -s "$spec" "notes/specs/$name"; then
      # The owning agent's canonical spec wins over a promoted or stale copy;
      # tagged witnesses still conflict because another agent could have
      # written one with the same name.
      if [ "$name" = "0x$a.json" ]; then
        cp "$spec" "notes/specs/$name" && echo "replaced $name"
      else
        echo "CONFLICT spec $name (canonical differs)"; fail=1; continue
      fi
    else
      cp "$spec" "notes/specs/$name"
    fi
    continue
    cp "$spec" "notes/specs/$name"
    case "$name" in
      0x[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]-*)
        mv "notes/specs/$name" "notes/specs/${name%%-*}.${name#*-}" ;;
    esac
  done
done

python3 scripts/normalize-specs.py >/dev/null
python3 scripts/gen-registry.py >/dev/null

for addr in "$@"; do
  ok=0; bad=0
  for spec in notes/specs/$addr.json notes/specs/$addr.*.json; do
    [ -f "$spec" ] || continue
    out="$(./scripts/verify-function.sh "$addr" analysis/lemans24.bin "$spec" 2>&1)"
    if grep -q '^ok ' <<<"$out"; then ok=$((ok+1)); else
      bad=$((bad+1))
      echo "FAIL $addr $(basename "$spec"): $(grep -m1 -E 'difference|!=|no port|oracle' <<<"$out" | cut -c1-90)"
    fi
  done
  echo "$addr ok=$ok fail=$bad"
  [ "$bad" -eq 0 ] || fail=1
done

[ "$fail" -eq 0 ] || { echo "merge-fixes: FAILURES above"; exit 1; }
echo "merge-fixes: all green"
