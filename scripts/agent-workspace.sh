#!/usr/bin/env bash
# Create an isolated copy of the porting harness for one agent.
#
# Usage: scripts/agent-workspace.sh <name>
#   -> prints the workspace path (/tmp/lm-<name>)
#
# The workspace shares the read-only analysis artifacts by symlink and owns its
# own copy of the Rust sources, specs and verifications. Two agents can build
# and verify at the same time without touching each other's target dirs, and a
# broken port in one workspace cannot fail another's build.
set -euo pipefail

NAME="${1:?usage: agent-workspace.sh <name>}"
SRC="${LEMANS_SOURCE:-$HOME/Projects/lemans-dc}"
DST="/tmp/lm-$NAME"

rm -rf "$DST"
mkdir -p "$DST/notes/specs" "$DST/notes/verifications"

ln -s "$SRC/analysis" "$DST/analysis"
ln -s "$SRC/fixtures" "$DST/fixtures"
ln -s "$SRC/work" "$DST/work"

cp -r "$SRC/scripts" "$DST/scripts"
cp -r "$SRC/tools" "$DST/tools"
cp -r "$SRC/rust" "$DST/rust"
rm -rf "$DST/rust/lemans-verify/target" "$DST/rust/sh4-oracle/target"

cp "$SRC"/notes/specs/*.json "$DST/notes/specs/"
cp "$SRC"/notes/specs/README.md "$DST/notes/specs/"

printf 'address\tname\tside_effects\tnotes\n' > "$DST/notes/ledger.tsv"

echo "$DST"
