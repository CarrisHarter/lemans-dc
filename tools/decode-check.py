#!/usr/bin/env python3
"""Differential-check the oracle's decoder against objdump's listing.

The listing is the ground truth for how an opcode is meant to be read. Two
classes of mistake are cheap to make and expensive to notice, so they are
checked exhaustively over every instance in the binary:

  1. access width  — the low nibble of the `mov` families encodes byte/word/
                     long as 0/1/2 or 4/5/6 or C/D/E, and the mnemonic's
                     suffix says which.
  2. displacement  — 0x80/0x81/0x84/0x85 put Rn/Rm in the m field and the
                     4-bit displacement in the low nibble.
  3. branch target — bra/bsr carry a signed 12-bit displacement; leaving
                     bit 11 positive sends every backward branch forward.
  4. indexed FPU   — fmov @(R0,Rm),FRn puts the address register in m and the
                     FR in n; the store form swaps them.
  5. fsts/flds/ftrc — the FR is the n field and the low byte is the opcode.
                     Verified with sh-elf-as: `ftrc fr4,fpul` -> 0xF43D.

Usage: tools/decode-check.py [disassembly.asm]
Exit code 1 on any mismatch.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LINE = re.compile(r"^([0-9a-f]{8}):\t((?:[0-9a-f]{2} )+)\s*\t\s*(\S+)\s*(.*)$")
DISP_OPS = re.compile(r"^(r\d+),@\((\d+),(r\d+)\)$|^@\((\d+),(r\d+)\),(r\d+)$")
FPU_LOAD = re.compile(r"^@\(r0,r(\d+)\),fr(\d+)$")
FPU_STORE = re.compile(r"^fr(\d+),@\(r0,r(\d+)\)$")
FPU_REG = re.compile(r"^fr(\d+),fpul$|^fpul,fr(\d+)$")

WIDTH = {
    0x0: "b", 0x4: "b", 0xC: "b",
    0x1: "w", 0x5: "w", 0xD: "w",
    0x2: "l", 0x6: "l", 0xE: "l",
}


def main(argv):
    path = Path(argv[1]) if len(argv) > 1 else ROOT / "analysis/export/disassembly.asm"
    width_checked = width_bad = 0
    disp_checked = disp_bad = 0
    branch_checked = branch_bad = 0
    fpu_checked = fpu_bad = 0
    fpul_checked = fpul_bad = 0

    with path.open(errors="replace") as fh:
        for line in fh:
            m = LINE.match(line.rstrip("\n"))
            if not m:
                continue
            mnemonic = m.group(3)
            raw = bytes(int(x, 16) for x in m.group(2).split())
            if len(raw) != 2:
                continue
            op = int.from_bytes(raw, "little")

            if mnemonic.startswith("mov.") and (op >> 12) in (0x0, 0x2, 0x6):
                low = op & 0xF
                if low in WIDTH:
                    width_checked += 1
                    if mnemonic != f"mov.{WIDTH[low]}":
                        width_bad += 1
                        if width_bad <= 5:
                            print(f"width  {m.group(1)} 0x{op:04x} {mnemonic} "
                                  f"{m.group(4).strip()} (nibble {low:x})")

            if mnemonic.startswith("fmov") and (op >> 12) == 0xF and (op & 0xF) in (6, 7):
                ops = m.group(4).strip()
                n, mm = (op >> 8) & 0xF, (op >> 4) & 0xF
                if (op & 0xF) == 6:
                    pm = FPU_LOAD.match(ops)
                    if pm:
                        fpu_checked += 1
                        if int(pm.group(1)) != mm or int(pm.group(2)) != n:
                            fpu_bad += 1
                            if fpu_bad <= 5:
                                print(f"fpu    {m.group(1)} 0x{op:04x} {mnemonic} {ops}")
                else:
                    pm = FPU_STORE.match(ops)
                    if pm:
                        fpu_checked += 1
                        if int(pm.group(1)) != mm or int(pm.group(2)) != n:
                            fpu_bad += 1
                            if fpu_bad <= 5:
                                print(f"fpu    {m.group(1)} 0x{op:04x} {mnemonic} {ops}")

            if mnemonic in ("fsts", "flds", "ftrc"):
                pm = FPU_REG.match(m.group(4).strip())
                if pm:
                    reg = int(pm.group(1) or pm.group(2))
                    fpul_checked += 1
                    if reg != ((op >> 8) & 0xF):
                        fpul_bad += 1
                        if fpul_bad <= 5:
                            print(f"fpul   {m.group(1)} 0x{op:04x} {mnemonic} "
                                  f"{m.group(4).strip()}")

            if mnemonic in ("bra", "bsr"):
                pc = int(m.group(1), 16)
                disp = ((op & 0x0FFF) << 4) & 0xFFFF
                disp = disp - 0x10000 if disp & 0x8000 else disp
                target = (pc + 4 + (disp >> 4) * 2) & 0xFFFFFFFF
                tm = re.search(r"0x([0-9a-f]+)", m.group(4))
                if tm:
                    branch_checked += 1
                    if target != int(tm.group(1), 16):
                        branch_bad += 1
                        if branch_bad <= 5:
                            print(f"branch {m.group(1)} 0x{op:04x} {mnemonic} "
                                  f"{m.group(4).strip()} -> 0x{target:08x}")

            if mnemonic in ("mov.b", "mov.w") and (op >> 8) in (0x80, 0x81, 0x84, 0x85):
                pm = DISP_OPS.match(m.group(4).strip())
                if not pm:
                    continue
                if pm.group(1):
                    disp, reg = int(pm.group(2)), int(pm.group(3)[1:])
                else:
                    disp, reg = int(pm.group(4)), int(pm.group(5)[1:])
                scale = 2 if (op >> 8) in (0x81, 0x85) else 1
                disp_checked += 1
                if (op & 0xF) * scale != disp or ((op >> 4) & 0xF) != reg:
                    disp_bad += 1
                    if disp_bad <= 5:
                        print(f"disp   {m.group(1)} 0x{op:04x} {mnemonic} "
                              f"{m.group(4).strip()}")

    print(f"width:  {width_checked} checked, {width_bad} mismatches")
    print(f"disp:   {disp_checked} checked, {disp_bad} mismatches")
    print(f"branch: {branch_checked} checked, {branch_bad} mismatches")
    print(f"fpu:    {fpu_checked} checked, {fpu_bad} mismatches")
    print(f"fpul:   {fpul_checked} checked, {fpul_bad} mismatches")
    return 1 if (width_bad or disp_bad or branch_bad or fpu_bad or fpul_bad) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
