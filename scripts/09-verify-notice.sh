#!/usr/bin/env bash
# Verify the published notice still matches the wording Atari SA approved,
# comparing whitespace-normalised text so line wrapping does not matter.
set -euo pipefail

CANON="This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA or any of its licensors. All rights in Le Mans 24 Hours remain with their respective owners."
FILE="${1:-$HOME/Projects/lemans-dc/analysis/export/NOTICE.txt}"

normalise() { tr '\n' ' ' | tr -s ' ' | sed 's/ *$//'; }

actual="$(head -2 "$FILE" | normalise)"
if [ "$actual" = "$CANON" ]; then
  echo "ok   NOTICE.txt matches the approved wording"
  exit 0
fi
echo "FAIL NOTICE.txt does not match the approved wording" >&2
printf '  approved: %s\n' "$CANON" >&2
printf '  actual:   %s\n' "$actual" >&2
exit 1
