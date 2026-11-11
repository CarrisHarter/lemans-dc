#!/usr/bin/env bash
# Step 1: preflight. Proves the toolchain is present before any disc work.
set -uo pipefail

ROOT="$HOME/Projects/lemans-dc"
GH="${GHIDRA_INSTALL_DIR:-$HOME/opt/ghidra_12.1.4_PUBLIC}"
JH="${JAVA_HOME:-$HOME/opt/jdk-21.0.13+9}"
fail=0

check() { # name, command, hint
  local name="$1" cmd="$2" hint="${3:-}"
  if eval "$cmd" >/dev/null 2>&1; then
    printf '  ok    %s\n' "$name"
  else
    printf '  FAIL  %s   %s\n' "$name" "$hint"
    fail=1
  fi
}

echo "== host"
check "node >= 26.12"        'node -e "process.exit(parseInt(process.versions.node)>=26?0:1)"' "mise install node@26"
check "python3"              'python3 -V'
check "ps (procps)"          'ps -o pid= -p $$'

echo "== Ghidra"
check "GHIDRA_INSTALL_DIR"   "[ -x $GH/support/analyzeHeadless ]" "install Ghidra 12.x to ~/opt"
check "JDK 21"               "$JH/bin/java -version" "install a JDK 21 to ~/opt"
check "SuperH4 language"     "grep -q 'SuperH4:LE:32:default' $GH/Ghidra/Processors/SuperH4/data/languages/SuperH4.ldefs" \
      "Ghidra 12.x ships it; older/newer layouts may name it differently"
check "decompiler native"    "[ -x $GH/Ghidra/Features/Decompiler/os/linux_x86_64/decompile ]"

echo "== SH-4 toolchain"
check "sh4-elf-objdump"      'sh4-elf-objdump --version' "pacman -S sh4-elf-binutils"
check "sh-elf-gcc (optional)" "[ -x $HOME/opt/sh-elf/bin/sh-elf-gcc ]" "scripts/07-build-sh4-cross-gcc.sh (needed for linkable C)"

echo "== rea"
check "rea fork"             "[ -f $HOME/Projects/rea-sh4/scripts/rea.mjs ]" "clone morluto/rea to ~/Projects/rea-sh4"
check "rea sh4 lane"         "grep -q 'SuperH4:LE:32:default' $HOME/Projects/rea-sh4/src/ghidra/GhidraRawBinaryLanguage.ts" \
      "apply notes/rea-sh4-sh4-lane.patch"
check "rea MCP registered"   "grep -q '\"rea\"' $HOME/.pi/agent/mcp.json"

echo "== inputs"
if find "$ROOT/work/raw" -maxdepth 1 -type f -name '*.gdi' 2>/dev/null | grep -q .; then
  echo "  ok    disc image present in work/raw"
else
  echo "  note  no disc image in $ROOT/work/raw (steps 2-4 and 11 wait on it)"
fi

exit "$fail"
