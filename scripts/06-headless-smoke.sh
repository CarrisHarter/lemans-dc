#!/usr/bin/env bash
# Step 6: prove the SH-4 lane works before any real disc image is involved.
# Uses the hand-assembled fixture (56 bytes) mapped at the Dreamcast load base.
set -euo pipefail

GH="${GHIDRA_INSTALL_DIR:-$HOME/opt/ghidra_12.1.4_PUBLIC}"
JAVA_HOME="${JAVA_HOME:-$HOME/opt/jdk-21.0.13+9}"
export GHIDRA_INSTALL_DIR="$GH" JAVA_HOME

PROJ_DIR="$HOME/Projects/lemans-dc/ghidra-proj"
HERE="$HOME/Projects/lemans-dc"
LANG_ID="SuperH4:LE:32:default"   # NOT SuperH:LE:32:SH-4 (that id does not exist in Ghidra 12.x)
BASE="0x8C010000"

cd "$HERE"
python3 scripts/make_sh4_fixture.py fixtures/sh4_fixture.bin >/dev/null
mkdir -p "$PROJ_DIR"
rm -rf "$PROJ_DIR/fixture" "$PROJ_DIR/fixture.rep" "$PROJ_DIR/fixture.lock" "$PROJ_DIR/fixture.lock~"

"$GH/support/analyzeHeadless" "$PROJ_DIR" fixture \
  -import fixtures/sh4_fixture.bin \
  -processor "$LANG_ID" -cspec default \
  -loader BinaryLoader -loader-baseAddr "$BASE" \
  -analysisTimeoutPerFile 600 \
  -scriptPath scripts \
  -postScript DumpDecomp.java "$BASE" 0x8C010010 \
  -deleteProject \
  | sed -n '/Execute script/,$p'
