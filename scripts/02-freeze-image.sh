#!/usr/bin/env bash
# Step 2: freeze a known-good baseline disc image and every file it references.
# Usage: 02-freeze-image.sh /path/to/image.{gdi,cue,iso,cdi,zip,7z}
# Copies read-only into work/raw, hashes each file, refuses to overwrite.
set -euo pipefail

SRC="${1:?usage: 02-freeze-image.sh /path/to/image}"
[ -f "$SRC" ] || { echo "not a file: $SRC" >&2; exit 1; }

WORK="$HOME/Projects/lemans-dc/work/raw"
mkdir -p "$WORK"
BASE="$(basename "$SRC")"
DIR="$(cd "$(dirname "$SRC")" && pwd)"

case "${BASE,,}" in
  *japan*|*jp*) REGION="Japan release" ;;
  *usa*|*"test drive"*) REGION="USA release (Test Drive Le Mans)" ;;
  *europe*|*"pal"*) REGION="Europe release" ;;
  *) REGION="unknown — confirm from IP.BIN product number" ;;
esac

# Files the descriptor names, so an archive is not the only thing frozen.
collect() {
  echo "$SRC"
  case "${BASE,,}" in
    *.gdi)
      awk 'NF >= 5 && $1 ~ /^[0-9]+$/ {
             name = $5
             for (i = 6; i < NF; i++) name = name " " $i
             print name
           }' "$SRC" | while IFS= read -r name; do
        [ -f "$DIR/$name" ] && echo "$DIR/$name"
      done
      ;;
    *.cue)
      sed -n 's/^FILE "\(.*\)" BINARY$/\1/p' "$SRC" | while IFS= read -r name; do
        [ -f "$DIR/$name" ] && echo "$DIR/$name"
      done
      ;;
  esac
}

mapfile -t FILES < <(collect)

for f in "${FILES[@]}"; do
  b="$(basename "$f")"
  if [ -e "$WORK/$b" ]; then
    echo "$WORK/$b already exists; refusing to overwrite a frozen baseline." >&2
    (cd "$WORK" && sha256sum "$b") >&2
    exit 1
  fi
done

for f in "${FILES[@]}"; do
  cp --preserve=timestamps "$f" "$WORK/"
  chmod a-w "$WORK/$(basename "$f")"
  echo "  frozen $(basename "$f")  $(stat -c%s "$f") bytes"
done

cd "$WORK"
{
  echo "# baseline frozen $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "# source: $SRC"
  echo "# region: $REGION"
  sha256sum -- *.gdi *.cue *.iso *.cdi *.bin *.raw *.7z *.zip 2>/dev/null || true
} > "$BASE.sha256"

echo
echo "=== $BASE.sha256"
cat "$BASE.sha256"
echo

case "${BASE,,}" in
  *.zip|*.7z)
    echo "== extracting archive (read-only copies)"
    rm -rf "$WORK/extracted"
    mkdir -p "$WORK/extracted"
    if command -v 7z >/dev/null 2>&1; then
      7z x -y -o"$WORK/extracted" "$WORK/$BASE" >/dev/null
    else
      unzip -q -o "$WORK/$BASE" -d "$WORK/extracted"
    fi
    chmod -R a-w "$WORK/extracted"
    (cd "$WORK/extracted" && sha256sum ./* | tee SHA256SUMS)
    echo
    ;;
esac

echo "Next: scripts/03-unpack-gdrom.py $(ls "$WORK"/extracted/*.cue "$WORK"/extracted/*.gdi "$WORK"/extracted/*.iso 2>/dev/null | head -1 || echo "$WORK/$BASE")"
