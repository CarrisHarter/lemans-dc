#!/usr/bin/env bash
# Step 13: export the deliverable for one target.
#
# Produces, under analysis/export/:
#   NOTICE.txt            approved attribution notice (permission condition 3)
#   pseudocode/<addr>.c   one file per function (rea decompile-all)
#   manifest.json         full rea evidence document for the export
#   symbols.tsv           address / name / status / file / body ranges
#   disassembly.asm       independent SH-4 listing via sh4-elf-objdump
#
# Usage: 08-export.sh <target.bin> <entry-address>
set -euo pipefail

TARGET="${1:?usage: 08-export.sh <target.bin> <entry-address>}"
ENTRY="${2:?usage: 08-export.sh <target.bin> <entry-address>}"
[ -f "$TARGET" ] || { echo "no such file: $TARGET" >&2; exit 1; }

ROOT="$HOME/Projects/lemans-dc"
OUT="$ROOT/analysis/export"
REA="$HOME/Projects/rea-sh4/scripts/rea.mjs"
export GHIDRA_INSTALL_DIR="${GHIDRA_INSTALL_DIR:-$HOME/opt/ghidra_12.1.4_PUBLIC}"
export JAVA_HOME="${JAVA_HOME:-$HOME/opt/jdk-21.0.13+9}"

mkdir -p "$OUT"
rm -f "$OUT/manifest.json" "$OUT/symbols.tsv" "$OUT/disassembly.asm"

# Permission condition 3: approved wording, first line of every published export.
cat > "$OUT/NOTICE.txt" <<'NOTICE'
This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA
or any of its licensors. All rights in Le Mans 24 Hours remain with their respective owners.

Non-commercial static analysis only. No ROM, disc image, or extracted asset (art, audio,
model, texture, video) is distributed here. Pseudocode below is Ghidra decompiler output,
not original source, and does not permit playing the game without a purchased copy.
NOTICE

echo "== decompile-all"
node "$REA" decompile-all "$TARGET" \
  --output "$OUT/pseudocode" \
  --layout address \
  --target-format raw-binary --architecture sh4 --entry-point "$ENTRY" \
  --provider ghidra --json > "$OUT/manifest.json"

echo "== symbols.tsv"
python3 - "$OUT/pseudocode/decompile-index.json" "$OUT/symbols.tsv" <<'PY'
import json, pathlib, sys
index = json.loads(pathlib.Path(sys.argv[1]).read_text())
rows = index.get("functions", [])
with open(sys.argv[2], "w") as fh:
    fh.write("address\tname\tstatus\tfile\tranges\n")
    for fn in sorted(rows, key=lambda r: r.get("address", "")):
        body = fn.get("body") or {}
        ranges = ";".join(
            f"{r.get('start')}-{r.get('end')}" for r in (body.get("ranges") or [])
        )
        fh.write(
            f"{fn.get('address','')}\t{fn.get('name','')}\t{fn.get('status','')}\t"
            f"{fn.get('relative_path','')}\t{ranges}\n"
        )
print(f"  {len(rows)} functions")
PY

echo "== disassembly (independent decoder)"
if command -v sh4-elf-objdump >/dev/null 2>&1; then
  sh4-elf-objdump -D -b binary -m sh4 -EL --adjust-vma="$ENTRY" "$TARGET" \
    > "$OUT/disassembly.asm"
  echo "  $(grep -cE '^[0-9a-f]+:' "$OUT/disassembly.asm") instructions"
else
  echo "  sh4-elf-objdump missing (pacman -S sh4-elf-binutils); skipping" >&2
fi

echo
ls -1 "$OUT"
