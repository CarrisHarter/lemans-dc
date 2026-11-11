#!/usr/bin/env python3
"""Census of SH-4 mnemonics in an objdump listing, against oracle coverage.

The point is to stop guessing what the interpreter needs next: the binary says
which instructions it actually executes, weighted by frequency.

Usage: opcode-census.py <disassembly.asm> [--oracle rust/sh4-oracle/src/lib.rs]
"""
from __future__ import annotations

import argparse
import collections
import pathlib
import re
import sys

LINE = re.compile(r"^\s*[0-9a-f]+:\s+(?:[0-9a-f]{2} )+\s*\t?([a-z][a-z0-9._/]*)")

# Mnemonics the oracle currently executes, by name (forms may still differ).
IMPLEMENTED = {
    "nop", "rts", "clrt", "sett",
    # data movement
    "mov", "mov.b", "mov.w", "mov.l", "mova",
    # arithmetic / logic
    "add", "sub", "and", "or", "xor", "tst",
    "cmp/eq", "cmp/hs", "cmp/ge", "cmp/hi", "cmp/gt", "cmp/pl", "cmp/pz",
    "dt", "extu.b", "extu.w", "exts.b", "exts.w",
    "shll", "shlr", "shll2", "shlr2", "shll8", "shlr8", "shll16", "shlr16",
    "not", "neg", "stc", "ldc", "fsts", "flds", "ftrc", "float",
    # control flow
    "jsr", "jmp", "bsr", "bra", "bt", "bf", "bt.s", "bf.s", "lds", "sts",
    "braf", "bsrf",
    # system registers and multiply/accumulate
    "lds.l", "sts.l", "ldc.l", "stc.l", "clrmac", "clrs", "sets", "movt",
    "mul.l", "mulu.w", "muls.w", "dmulu.l", "dmuls.l", "mac.w", "mac.l",
    "div0s", "div0u", "tas.b", "xtrct", "cmp/str",
    # carry / overflow / rotate / shift-by-register
    "addc", "addv", "subc", "subv", "negc", "shad", "shld", "shal", "shar",
    "rotl", "rotr", "rotcl", "rotcr", "swap.b", "swap.w",
    # gbr-relative byte logic and cache hints
    "tst.b", "and.b", "or.b", "xor.b", "pref", "ocbi", "ocbp", "ocbwb", "ldtlb",
    "movca.l",
    # fpu
    "fadd", "fsub", "fmul", "fdiv", "fmac", "fmov", "fmov.s", "fldi0", "fldi1",
    "fcmp/eq", "fcmp/gt", "fneg", "fabs", "fsqrt",
}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("listing", type=pathlib.Path)
    ap.add_argument("--top", type=int, default=25)
    args = ap.parse_args()

    counts: collections.Counter[str] = collections.Counter()
    total = 0
    with args.listing.open(errors="replace") as fh:
        for line in fh:
            if line.startswith(" ") and ":" in line[:10]:
                pass
            m = LINE.match(line)
            if not m:
                continue
            counts[m.group(1)] += 1
            total += 1

    if not total:
        print("no instructions parsed", file=sys.stderr)
        return 1

    covered = sum(n for m, n in counts.items() if m in IMPLEMENTED)
    print(f"instructions        {total}")
    print(f"distinct mnemonics  {len(counts)}")
    print(f"oracle covers       {covered/total*100:.2f}% of static instructions"
          f" ({covered}/{total})")
    print()
    print(f"{'mnemonic':<14}{'count':>10}{'share':>9}  oracle")
    for mnemonic, n in counts.most_common(args.top):
        mark = "yes" if mnemonic in IMPLEMENTED else "NO"
        print(f"{mnemonic:<14}{n:>10}{n/total*100:>8.2f}%  {mark}")

    missing = [(m, n) for m, n in counts.items() if m not in IMPLEMENTED]
    print()
    print("biggest gaps:")
    for mnemonic, n in sorted(missing, key=lambda kv: -kv[1])[:15]:
        print(f"  {mnemonic:<14}{n:>10}  ({n/total*100:.2f}%)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
