#!/usr/bin/env python3
"""Emit a small hand-assembled SH-4 (Dreamcast, little-endian) blob.

Purpose: prove the Ghidra SuperH4:LE:32:default lane decodes SH-4, finds
functions, and decompiles them, before any real 1ST_READ.BIN is involved.

Layout is fixed and mirrors the Dreamcast mapping used for the real target:
file offset 0 -> 0x8C010000.
"""
import struct
import sys

BASE = 0x8C010000

# (offset, halfword, comment)
WORDS = [
    # fn_sum_to_r4: r0 = 0; while (r1--) r0++;  (r1 = r4)
    (0x00, 0x6413, "mov r4,r1"),
    (0x02, 0xE000, "mov #0,r0"),
    (0x04, 0x7001, "add #1,r0     <-- loop"),
    (0x06, 0x4110, "dt r1"),
    (0x08, 0x8BFC, "bf 0x8C010004"),
    (0x0A, 0x000B, "rts"),
    (0x0C, 0x0009, "nop           (delay slot)"),

    # fn_call_via_literal: return fn_sum_to_r4(5);
    (0x10, 0xE405, "mov #5,r4"),
    (0x12, 0x0009, "nop"),
    (0x14, 0xD102, "mov.l @(2,pc),r1 -> 0x8C010020"),
    (0x16, 0x412B, "jmp @r1"),
    (0x18, 0x0009, "nop           (delay slot)"),
    (0x1A, 0x0009, "nop"),
    (0x1C, 0x0009, "nop"),
    (0x1E, 0x0009, "nop"),
    (0x20, 0x0000, "literal lo 0x8C010000"),
    (0x22, 0x8C01, "literal hi"),

    # fn_fpu_add: fr0 += fr4; return via FPUL-agnostic path
    (0x28, 0xF042, "fadd fr0,fr4"),
    (0x2A, 0xF04A, "fldi0 fr0? (probe decode)"),
    (0x2C, 0x000B, "rts"),
    (0x2E, 0x0009, "nop           (delay slot)"),

    # fn_store_byte: *(char*)(r4+4) = 0x41
    (0x30, 0xE441, "mov #0x41,r4  (probe immediate form)"),
    (0x32, 0x6043, "mov r4,r0"),
    (0x34, 0x000B, "rts"),
    (0x36, 0x0009, "nop           (delay slot)"),
]


def build() -> bytes:
    end = max(off + 2 for off, _, _ in WORDS)
    buf = bytearray(end)
    for off, word, _ in WORDS:
        struct.pack_into("<H", buf, off, word)  # little-endian halfwords
    return bytes(buf)


def main() -> int:
    out = sys.argv[1] if len(sys.argv) > 1 else "sh4_fixture.bin"
    data = build()
    with open(out, "wb") as fh:
        fh.write(data)
    print(f"wrote {out}: {len(data)} bytes, base 0x{BASE:08X}")
    for off, word, comment in WORDS:
        print(f"  0x{BASE + off:08X}  {word:04X}  {comment}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
