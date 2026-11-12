//! 0x8c017c0c — decode a 1..4 byte little-endian integer. The low 3 bits of
//! the first byte select the length; the value is the first byte shifted
//! right by the length, with the following bytes packed above it. The
//! follow-up bytes are read with group-8 `mov.b @(disp,Rm),R0` ops; the
//! oracle decodes the base register from bits 7-4 and the displacement from
//! bits 3-0, so 0x8441/0x8442/0x8443 read r4+1, r4+2, r4+3 — the addresses the
//! disassembler prints.
//!
//! ABI: r4 = src, r5 = u32* out. Returns r4 advanced past the consumed bytes.
//!
//! Side effects the decompiled C does not show: r6 always ends as the decoded
//! value, while r3/r1 keep the in-place shifts of the path taken — 3-byte:
//! r3 = b1<<8, r1 = b2; 4-byte: r3 = b1<<16, r1 = b2<<8. T comes from the
//! last shift executed — always 0: the short paths end in `shar r6` whose carry
//! bit is already known clear, and the 3/4-byte paths end in `shll8` of a
//! zero-extended byte, whose bit 24 is 0. The returned pointer is produced by
//! `mov r4,r0` in each branch's delay slot, i.e. after the `add #n,r4`, and
//! r4 itself ends advanced.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let r5 = cpu.r[5];

    // mov.b @r4,r0 ; exts.b r0,r6
    let b0 = mem.read_u8(r4).unwrap();
    let r6 = b0 as i8 as i32 as u32;

    if b0 & 1 == 0 {
        // 1-byte: shar r6 ; add #1,r4 ; mov.l r6,@r5 ; bra (delay mov r4,r0)
        let v = ((r6 as i32) >> 1) as u32;
        r4 = r4.wrapping_add(1);
        mem.write_u32(r5, v).unwrap();
        cpu.r[6] = v;
        cpu.r[4] = r4;
        cpu.r[0] = r4;
        cpu.set_t(false); // shar shifted out bit 0, which is 0 on this path
        return;
    }

    // mov.b @(1,r4),r0 ; extu.b r0,r3
    let b1 = mem.read_u8(r4.wrapping_add(1)).unwrap() as u32;
    if b0 & 2 == 0 {
        // 2-byte: shar shar shll8 ; or ; add #2,r4 ; store ; bra (delay mov r4,r0)
        let mut v = ((r6 as i32) >> 1) as u32;
        v = ((v as i32) >> 1) as u32;
        v = v << 8;
        v |= b1;
        r4 = r4.wrapping_add(2);
        mem.write_u32(r5, v).unwrap();
        cpu.r[6] = v;
        cpu.r[3] = b1;
        cpu.r[4] = r4;
        cpu.r[0] = r4;
        cpu.set_t(false); // second shar shifted out bit 1 == 0 on this path
        return;
    }

    // mov.b @(2,r4),r0 ; extu.b r0,r1
    let b2 = mem.read_u8(r4.wrapping_add(2)).unwrap() as u32;
    if b0 & 4 == 0 {
        // 3-byte: shar x3, shll16, shll8 r3, or r1 ; add #3,r4 ; store
        let mut v = ((r6 as i32) >> 1) as u32;
        v = ((v as i32) >> 1) as u32;
        v = ((v as i32) >> 1) as u32;
        v = v << 16;
        v |= b1 << 8;
        v |= b2;
        r4 = r4.wrapping_add(3);
        mem.write_u32(r5, v).unwrap();
        cpu.r[6] = v;
        cpu.r[3] = b1 << 8; // shll8 r3 (3-byte path, before the or)
        cpu.r[1] = b2; // extu.b r0,r1; the shll8 r1 is 4-byte-only
        cpu.r[4] = r4;
        cpu.r[0] = r4;
        cpu.set_t(false); // third shar shifted out bit 2 == 0 on this path
        return;
    }

    // 4-byte: shar x3, shll16+shll8, shll16 r3, shll8 r1, or b3
    let b3 = mem.read_u8(r4.wrapping_add(3)).unwrap() as u32;
    let mut v = ((r6 as i32) >> 1) as u32;
    v = ((v as i32) >> 1) as u32;
    v = ((v as i32) >> 1) as u32;
    v = v << 16;
    v = v << 8;
    v |= b1 << 16;
    v |= b2 << 8;
    v |= b3;
    r4 = r4.wrapping_add(4);
    mem.write_u32(r5, v).unwrap();
    cpu.r[6] = v;
    cpu.r[3] = b1 << 16; // shll16 r3
    cpu.r[1] = b2 << 8; // shll8 r1
    cpu.r[4] = r4;
    cpu.r[0] = r4;
    cpu.set_t(false); // last shift is shll8 r1 with r1 = b2, so bit24 is 0
}
