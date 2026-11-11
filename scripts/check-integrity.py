#!/usr/bin/env python3
"""Prove the ledger, the ports and the evidence agree.

Every claimed port must have all three: a `p_<addr>.rs`, a verification file
whose `result` is `pass`, and a ledger row. A row without a port is a claim
with nothing behind it; a port without a row is unrecorded work.

Usage: scripts/check-integrity.py [--quiet]
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PORTS = ROOT / "rust/lemans-verify/src/ports"
SPECS = ROOT / "notes/specs"
VERIFS = ROOT / "notes/verifications"
LEDGER = ROOT / "notes/address-map.md"
NAME = re.compile(r"^p_([0-9a-f]{8})\.rs$")


def main(argv):
    quiet = "--quiet" in argv
    ports = {m.group(1) for p in PORTS.glob("p_*.rs") if (m := NAME.match(p.name))}

    lines = LEDGER.read_text().splitlines()
    start = next(i for i, l in enumerate(lines) if l.strip() == "## Verified ports")
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("## ")), len(lines))
    ledger = {}
    for line in lines[start:end]:
        m = re.match(r"^\| `0x([0-9a-f]{8})` \|", line)
        if m:
            ledger[m.group(1)] = line

    problems = []
    for addr in sorted(set(ledger) - ports):
        problems.append(f"ledger row without a port: 0x{addr}")
    for addr in sorted(ports - set(ledger)):
        problems.append(f"port without a ledger row: 0x{addr}")

    passing = 0
    for addr in sorted(ports):
        v = VERIFS / f"0x{addr}.json"
        if not v.exists():
            problems.append(f"port without evidence: 0x{addr}")
            continue
        try:
            doc = json.loads(v.read_text())
        except json.JSONDecodeError:
            problems.append(f"unreadable evidence: 0x{addr}")
            continue
        if doc.get("result") != "pass":
            problems.append(f"evidence says {doc.get('result')}: 0x{addr}")
        else:
            passing += 1

    blocked = 0
    if "## Blocked" in LEDGER.read_text():
        blocked = len(re.findall(r"^\| `0x[0-9a-f]{8}` \|", LEDGER.read_text().split("## Blocked", 1)[1].split("\n## ", 1)[0], re.M))

    if problems:
        for p in problems:
            print(f"FAIL {p}")
        print(f"ports {len(ports)}, ledger {len(ledger)}, passing evidence {passing}, blocked {blocked}, problems {len(problems)}")
        return 1
    if not quiet:
        print(f"ok: ports {len(ports)}, ledger {len(ledger)}, passing evidence {passing}, blocked {blocked}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
