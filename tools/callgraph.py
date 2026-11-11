#!/usr/bin/env python3
"""Classify every exported function as leaf or caller.

A leaf has no outgoing control flow: no `bsr`/`bsrf` to another function, no
`jsr`/`jmp @Rn` (indirect, so the target cannot be proven internal), and no
`bra`/`bt`/`bf` that leaves its own range. Those are the only functions a spec
can pin today, because the harness cannot follow a call.

Usage:
  tools/callgraph.py analysis/export/disassembly.asm analysis/export/pseudocode/functions
  tools/callgraph.py --leaves          # ascending, one address per line
  tools/callgraph.py --stats
"""
import bisect
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

LINE = re.compile(r"^([0-9a-f]{8}):\t(?:[0-9a-f]{2} +)+\t\s*(\S+)\s*(.*)$")
HEAD_ADDR = re.compile(r"^// address: (0x[0-9a-f]+)")
HEAD_RANGE = re.compile(r"^// range: (0x[0-9a-f]+)-(0x[0-9a-f]+)")

BRANCH = {"bra", "bsr", "bf", "bt", "bf/s", "bt/s", "bsrf"}
INDIRECT = {"jsr", "jmp"}


def parse_functions(fn_dir: Path):
    funcs = []
    for path in sorted(fn_dir.glob("*.c")):
        addr = end = None
        with path.open("r", errors="replace") as fh:
            for _ in range(12):
                line = fh.readline()
                if not line:
                    break
                m = HEAD_ADDR.match(line)
                if m:
                    addr = int(m.group(1), 16)
                m = HEAD_RANGE.match(line)
                if m:
                    addr = int(m.group(1), 16)
                    end = int(m.group(2), 16)
                if addr is not None and end is not None:
                    break
        if addr is None or end is None:
            continue
        funcs.append((addr, end, path.name))
    funcs.sort()
    return funcs


def load_instructions(asm: Path):
    """Read the whole disassembly once: [(pc, mnemonic, operands)] ascending."""
    out = []
    with asm.open("r", errors="replace") as fh:
        for line in fh:
            if not line or line[0] not in "0123456789abcdef":
                continue
            m = LINE.match(line.rstrip("\n"))
            if not m:
                continue
            out.append((int(m.group(1), 16), m.group(2), m.group(3)))
    out.sort()
    return out


def classify(instructions, funcs):
    pcs = [pc for pc, _, _ in instructions]
    rows = []
    for addr, end, name in funcs:
        lo = bisect.bisect_left(pcs, addr)
        hi = bisect.bisect_right(pcs, end)
        outgoing = []
        for pc, mnem, ops in instructions[lo:hi]:
            if mnem in INDIRECT:
                outgoing.append((pc, f"{mnem} {ops}".strip()))
                continue
            if mnem in BRANCH:
                target = None
                m = re.search(r"0x([0-9a-f]+)", ops)
                if m:
                    target = int(m.group(1), 16)
                elif ops.startswith("@"):
                    outgoing.append((pc, f"{mnem} {ops}".strip()))
                    continue
                if target is None:
                    continue
                if not (addr <= target <= end):
                    outgoing.append((pc, f"{mnem} 0x{target:08x}"))
        rows.append({
            "address": addr,
            "end": end,
            "name": name,
            "leaf": not outgoing,
            "outgoing": outgoing,
        })
    return rows


def main(argv):
    asm = ROOT / "analysis/export/disassembly.asm"
    fn_dir = ROOT / "analysis/export/pseudocode/functions"
    args = [a for a in argv[1:] if not a.startswith("--")]
    if len(args) >= 1:
        asm = Path(args[0])
    if len(args) >= 2:
        fn_dir = Path(args[1])

    funcs = parse_functions(fn_dir)
    rows = classify(load_instructions(asm), funcs)
    leaves = [r for r in rows if r["leaf"]]

    if "--leaves" in argv:
        for r in leaves:
            print(f"0x{r['address']:08x}")
        return 0
    if "--stats" in argv:
        print(f"functions   {len(rows)}")
        print(f"leaves      {len(leaves)}")
        print(f"callers     {len(rows) - len(leaves)}")
        return 0

    print("address\tend\tleaf\toutgoing\tname")
    for r in rows:
        first = r["outgoing"][0] if r["outgoing"] else ("", "")
        print(f"0x{r['address']:08x}\t0x{r['end']:08x}\t{int(r['leaf'])}\t"
              f"{len(r['outgoing'])}\t{r['name']}\t{first[0]}\t{first[1]}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
