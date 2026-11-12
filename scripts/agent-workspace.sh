#!/usr/bin/env bash
# Create an isolated copy of the porting harness for one agent.
#
# Usage: scripts/agent-workspace.sh <name>
#   -> prints the workspace path ($LEMANS_WORKSPACES/lm-<name>)
#
# The workspace shares the read-only analysis artifacts by symlink and owns its
# own copy of the Rust sources, specs and verifications. Two agents can build
# and verify at the same time without touching each other's target dirs, and a
# broken port in one workspace cannot fail another's build.
set -euo pipefail

NAME="${1:?usage: agent-workspace.sh <name>}"
SRC="${LEMANS_SOURCE:-$HOME/Projects/lemans-dc}"
# Scratch lives on disk, not tmpfs: /tmp is RAM and a reboot loses it. Do not
# put this under the repo's work/ — every workspace symlinks work/, so that
# would create a loop.
WS_ROOT="${LEMANS_WORKSPACES:-$HOME/Work/lemans-agents}"
mkdir -p "$WS_ROOT"
DST="$WS_ROOT/lm-$NAME"

rm -rf "$DST"
mkdir -p "$DST/notes/specs" "$DST/notes/verifications"

ln -s "$SRC/analysis" "$DST/analysis"
ln -s "$SRC/fixtures" "$DST/fixtures"
ln -s "$SRC/work" "$DST/work"

cp -r "$SRC/scripts" "$DST/scripts"
cp -r "$SRC/tools" "$DST/tools"
# Copy sources without build artifacts: 40 MB of target/ per workspace filled
# /tmp once 100 workspaces had accumulated.
mkdir -p "$DST/rust"
for crate in "$SRC"/rust/*/; do
  name="$(basename "$crate")"
  mkdir -p "$DST/rust/$name"
  cp -r "$crate/src" "$DST/rust/$name/" 2>/dev/null || true
  for f in Cargo.toml Cargo.lock build.rs; do
    [ -f "$crate/$f" ] && cp "$crate/$f" "$DST/rust/$name/"
  done
  cp -r "$crate/tests" "$DST/rust/$name/" 2>/dev/null || true
done
rm -rf "$DST/rust/lemans-verify/target" "$DST/rust/sh4-oracle/target"

cp "$SRC"/notes/specs/*.json "$DST/notes/specs/"
cp "$SRC"/notes/specs/README.md "$DST/notes/specs/"

printf 'address\tname\tside_effects\tnotes\n' > "$DST/notes/ledger.tsv"

echo "$DST"
