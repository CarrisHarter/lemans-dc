#!/usr/bin/env python3
"""Rank leaf functions by how ready they are for the spec/port/verify loop.

A leaf has no outgoing control flow (see tools/callgraph.py). This adds what
the oracle needs: which mnemonics it cannot execute yet, and whether the
function touches hardware addresses the harness does not model.

Usage: tools/leaf-worklist.py [--ready] [--needs-oracle] [--hw]
Output: TSV, ascending address.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import ast  # noqa: E402

import callgraph  # noqa: E402

_census = (Path(__file__).resolve().parent / "opcode-census.py").read_text()
_tree = ast.parse(_census)
IMPLEMENTED = next(
    ast.literal_eval(node.value)
    for node in _tree.body
    if isinstance(node, ast.Assign)
    and any(getattr(t, "id", "") == "IMPLEMENTED" for t in node.targets)
)

ROOT = Path(__file__).resolve().parent.parent
ASM = ROOT / "analysis/export/disassembly.asm"
FUNCS = ROOT / "analysis/export/pseudocode/functions"

# Addresses the oracle maps nothing at: hardware registers, BIOS, store queues.
HW_PREFIXES = ("ff00", "a05f", "8c00")

# System registers the CPU struct has no field for yet. A leaf that touches one
# cannot be pinned until the oracle and the spec format model it.
UNMODELLED = {"mach", "macl", "gbr", "vbr", "ssr", "spc", "sgr", "dbr",
              "fpscr", "r0_bank", "r1_bank", "r2_bank", "r3_bank",
              "r4_bank", "r5_bank", "r6_bank", "r7_bank"}


def main(argv):
    ins = callgraph.load_instructions(ASM)
    funcs = callgraph.parse_functions(FUNCS)
    rows = callgraph.classify(ins, funcs)
    pcs = [pc for pc, _, _ in ins]

    out = []
    for r in rows:
        if not r["leaf"]:
            continue
        lo = callgraph.bisect.bisect_left(pcs, r["address"])
        hi = callgraph.bisect.bisect_right(pcs, r["end"])
        body = ins[lo:hi]
        missing = sorted({m for pc, m, ops in body if m not in IMPLEMENTED})
        hw = sorted({
            ops.split("!")[1].strip()
            for pc, m, ops in body
            if "!" in ops and ops.split("!")[1].strip().startswith(HW_PREFIXES)
        })
        uses_fpu = any(m.startswith("f") and m != "ff" for _, m, _ in body)
        special = sorted({
            tok
            for _, m, ops in body
            if m.split(".")[0] in {"stc", "sts", "ldc", "lds"}
            for tok in ops.replace(",", " ").split()
            if tok in UNMODELLED
        })
        out.append({
            "address": r["address"],
            "end": r["end"],
            "insn": len(body),
            "missing": missing,
            "hw": hw,
            "fpu": uses_fpu,
            "special": special,
        })

    if "--ready" in argv:
        out = [r for r in out if not r["missing"] and not r["hw"] and not r["special"]]
    elif "--needs-oracle" in argv:
        out = [r for r in out if r["missing"] and not r["hw"]]
    elif "--hw" in argv:
        out = [r for r in out if r["hw"]]

    print("address\tend\tinsn\tfpu\tmissing\thw\tspecial")
    for r in out:
        print(f"0x{r['address']:08x}\t0x{r['end']:08x}\t{r['insn']}\t"
              f"{int(r['fpu'])}\t{','.join(r['missing'])}\t{','.join(r['hw'])}\t"
              f"{','.join(r['special'])}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
