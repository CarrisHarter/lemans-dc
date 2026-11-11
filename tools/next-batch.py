#!/usr/bin/env python3
"""Print the next batch of leaf functions that the current harness can pin.

Filters, in order: leaf (no outgoing control flow), every mnemonic implemented,
no hardware literal, no system register the CPU struct lacks, no `stc sr` /
`ldc sr` (SR is only compared bit 0 today), not already ported.

Usage: tools/next-batch.py [--count N] [--all]
"""
import ast
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import callgraph  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
ASM = ROOT / "analysis/export/disassembly.asm"
FUNCS = ROOT / "analysis/export/pseudocode/functions"
PORTS = ROOT / "rust/lemans-verify/src/ports"

HW_PREFIXES = ("ff00", "a05f", "8c00")
UNMODELLED = {
    "mach", "macl", "gbr", "vbr", "ssr", "spc", "sgr", "dbr", "fpscr",
    "r0_bank", "r1_bank", "r2_bank", "r3_bank",
    "r4_bank", "r5_bank", "r6_bank", "r7_bank",
}


def implemented():
    tree = ast.parse((ROOT / "tools/opcode-census.py").read_text())
    return next(
        ast.literal_eval(node.value)
        for node in tree.body
        if isinstance(node, ast.Assign)
        and any(getattr(t, "id", "") == "IMPLEMENTED" for t in node.targets)
    )


def ported():
    return {
        int(p.stem[2:], 16)
        for p in PORTS.glob("p_*.rs")
        if len(p.stem) == 10
    }


def main(argv):
    count = 10
    if "--count" in argv:
        count = int(argv[argv.index("--count") + 1])
    want_all = "--all" in argv
    skip = set()
    if "--skip" in argv:
        skip = {int(a, 16) for a in argv[argv.index("--skip") + 1].split(",") if a}

    impl = implemented()
    done = ported()
    ins = callgraph.load_instructions(ASM)
    funcs = callgraph.parse_functions(FUNCS)
    pcs = [pc for pc, _, _ in ins]

    picked = []
    for row in callgraph.classify(ins, funcs):
        if not row["leaf"] or row["address"] in done or row["address"] in skip:
            continue
        lo = callgraph.bisect.bisect_left(pcs, row["address"])
        hi = callgraph.bisect.bisect_right(pcs, row["end"])
        body = ins[lo:hi]
        if any(m not in impl for _, m, _ in body):
            continue
        if any("!" in ops and ops.split("!")[1].strip().startswith(HW_PREFIXES)
               for _, _, ops in body):
            continue
        special = {
            tok
            for _, m, ops in body
            if m.split(".")[0] in {"stc", "sts", "ldc", "lds"}
            for tok in ops.replace(",", " ").split()
            if tok in UNMODELLED
        }
        if special:
            continue
        if any(m in {"stc", "ldc"} and ops.replace(",", " ").split()[-1] == "sr"
               for _, m, ops in body):
            continue
        picked.append((row["address"], row["end"], len(body)))
        if not want_all and len(picked) >= count:
            break

    for addr, end, n in picked:
        print(f"0x{addr:08x}\t0x{end:08x}\t{n}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
