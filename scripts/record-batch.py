#!/usr/bin/env python3
"""Append verified ports to notes/address-map.md from evidence on disk.

Usage: scripts/record-batch.py <workspace> <address> [<address> ...]

An address is recorded only if `notes/verifications/<address>.json` exists and
says `"result": "pass"`. Name and side effects come from the workspace's
`notes/ledger.tsv`; the evidence column is generated from the verification file.
Idempotent: an address already in the table is skipped.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LEDGER = ROOT / "notes/address-map.md"
VERIFICATIONS = ROOT / "notes/verifications"

ROW = re.compile(r"^\| `0x[0-9a-f]{8}` \|")


def ledger_rows(ws: Path):
    rows = {}
    path = ws / "notes/ledger.tsv"
    if not path.exists():
        return rows
    for line in path.read_text().splitlines()[1:]:
        parts = line.split("\t")
        if len(parts) >= 3 and parts[0].startswith("0x"):
            rows[int(parts[0], 16)] = (parts[1], parts[2], parts[3] if len(parts) > 3 else "")
    return rows


def main(argv):
    if len(argv) < 3:
        print(__doc__)
        return 2
    ws = Path(argv[1])
    names = ledger_rows(ws)
    text = LEDGER.read_text()
    lines = text.splitlines()
    start = next(i for i, l in enumerate(lines) if l.strip() == "## Verified ports")
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("## ")),
               len(lines))
    existing = {
        int(m.group(1), 16)
        for m in (re.match(r"^\| `(0x[0-9a-f]{8})` \|", l) for l in lines[start:end])
        if m
    }

    added = []
    for arg in argv[2:]:
        addr = int(arg, 16)
        if addr in existing:
            continue
        evidence = VERIFICATIONS / f"0x{addr:08x}.json"
        if not evidence.exists():
            print(f"skip 0x{addr:08x}: no verification file", file=sys.stderr)
            continue
        doc = json.loads(evidence.read_text())
        if doc.get("result") != "pass":
            print(f"skip 0x{addr:08x}: verification says {doc.get('result')}", file=sys.stderr)
            continue
        name, side, _ = names.get(addr, ("unknown", "", ""))
        steps = doc.get("oracle", {}).get("steps", "?")
        row = (f"| `0x{addr:08x}` | `{name}` | "
               f"`notes/verifications/0x{addr:08x}.json` — {steps} oracle steps"
               + (f"; {side}" if side else "") + " |")
        added.append(row)

    if not added:
        print("nothing new to record")
        return 0

    last = max(i for i in range(start, end) if ROW.match(lines[i]))
    lines[last + 1:last + 1] = added
    LEDGER.write_text("\n".join(lines) + "\n")
    print(f"recorded {len(added)} port(s) in notes/address-map.md")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
