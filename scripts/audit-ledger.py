#!/usr/bin/env python3
"""Audit notes/address-map.md against the evidence on disk.

A ledger row is a claim. It is only true if `notes/verifications/<address>.json`
exists AND says `"result": "pass"`. An earlier version of this check only tested
existence, which let three failing ports sit in the ledger unnoticed.

Usage: scripts/audit-ledger.py [--quiet]
Exit code 1 if any row fails.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LEDGER = ROOT / "notes/address-map.md"
VERIFICATIONS = ROOT / "notes/verifications"
ROW = re.compile(r"^\| `(0x[0-9a-f]{8})`")


def main(argv):
    lines = LEDGER.read_text().splitlines()
    start = next(i for i, l in enumerate(lines) if l.strip() == "## Verified ports")
    end = next(
        (i for i in range(start + 1, len(lines)) if lines[i].startswith("## ")),
        len(lines),
    )
    rows = [l for l in lines[start:end] if ROW.match(l)]

    problems = []
    for row in rows:
        addr = ROW.match(row).group(1)
        path = VERIFICATIONS / f"{addr}.json"
        if not path.exists():
            problems.append((addr, "no evidence file"))
            continue
        result = json.loads(path.read_text()).get("result")
        if result != "pass":
            problems.append((addr, f"result={result}"))

    print(f"ledger rows {len(rows)}, not passing {len(problems)}")
    for addr, why in problems:
        print(f"  {addr}: {why}")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
