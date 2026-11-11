#!/usr/bin/env python3
"""List callers whose every resolvable callee is already ported.

A caller can be verified once its callees are: the oracle runs the real bytes
of the whole call tree, and the port calls the ported callee. The remaining
risk is a callee that touches memory the spec does not seed, which shows up as
an oracle error rather than a silent mismatch.

`jsr @Rn` is resolved through the most recent `mov.l <pool>,Rn` in the same
function, using the value the disassembler prints after `!`.

Usage:
  tools/caller-worklist.py [--count N] [--skip a,b,...]
"""
import bisect
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from callgraph import parse_functions, load_instructions, LINE  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
ASM = ROOT / "analysis/export/disassembly.asm"
FN = ROOT / "analysis/export/pseudocode/functions"
PORTS = ROOT / "rust/lemans-verify/src/ports"

POOL = re.compile(r"^\s*0x([0-9a-f]+),\s*(r\d+)\s*!\s*([0-9a-f]+)")
BSR = re.compile(r"^\s*0x([0-9a-f]+)")


def main(argv):
    limit = 40
    skip = set()
    if "--count" in argv:
        limit = int(argv[argv.index("--count") + 1])
    if "--skip" in argv:
        skip = {int(a, 16) for a in argv[argv.index("--skip") + 1].split(",") if a}

    import json
    verifs = ROOT / "notes/verifications"
    ported = set()
    for p in PORTS.glob("p_*.rs"):
        a = int(p.stem[2:], 16)
        v = verifs / f"0x{a:08x}.json"
        if not v.exists():
            continue
        try:
            if json.loads(v.read_text()).get("result") == "pass":
                ported.add(a)
        except Exception:
            pass
    funcs = parse_functions(FN)
    instructions = load_instructions(ASM)
    pcs = [pc for pc, _, _ in instructions]

    rows = []
    for addr, end, name in funcs:
        if addr in ported or addr in skip:
            continue
        lo = bisect.bisect_left(pcs, addr)
        hi = bisect.bisect_right(pcs, end)
        reg = {}
        callees = []
        unresolved = False
        for pc, mnem, ops in instructions[lo:hi]:
            if mnem == "mov.l":
                m = POOL.match(ops)
                if m:
                    reg[m.group(2)] = int(m.group(3), 16)
                continue
            if mnem == "bsr":
                m = BSR.match(ops)
                if m:
                    callees.append(int(m.group(1), 16))
                continue
            if mnem == "jsr":
                m = re.search(r"@(r\d+)", ops)
                if m and m.group(1) in reg:
                    callees.append(reg[m.group(1)])
                else:
                    unresolved = True
                continue
            if mnem == "jmp":
                unresolved = True
        if unresolved or not callees:
            continue
        if all(c in ported for c in callees):
            rows.append((addr, end - addr, len(callees), sorted(set(callees))))

    rows.sort()
    for addr, size, n, callees in rows[:limit]:
        print(f"0x{addr:08x}\t0x{addr + size:08x}\t{size}\t{n}\t{','.join(f'0x{c:08x}' for c in callees)}")
    print(f"# {len(rows)} callers with all callees ported", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
