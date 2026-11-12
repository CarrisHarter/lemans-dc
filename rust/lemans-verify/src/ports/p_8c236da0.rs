//! 0x8c236da0 — set a mode byte on a table entry.
//!
//! Saves `r14`/`r13`/`pr`, calls the table lookup 0x8c237780 with `r4`. On a
//! miss it returns the literal `0xeeee`; on a hit it builds a 3-bit mode from
//! `r5` (bits 0x2000000/0x1000000/0x400000, `and #7`) into the top byte of
//! `[[r4+0x88]+8]`, preserving the low 24 bits, and returns 0. T is whatever
//! the `shll8 r14` carry on the taken path (always 0, since r14 is masked to
//! 0..7 first); `and`/`or` do not touch it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15 ; add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);

    cpu.r[3] = mem.read_u32(0x8c236e24).unwrap_or(0x8c237780); // mov.l 0x8c236e24,r3
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.pr = 0x8c236db2; // jsr @r3
    cpu.r[14] = 1; // delay slot: mov #1,r14

    crate::registry::p_8c237780::run(cpu, mem);

    let hit = cpu.r[0] != 0; // tst r0,r0
    cpu.set_t(!hit);
    if !hit {
        // bf not taken: return [0x8c236e28] (0xeeee), branch to the epilogue.
        cpu.r[0] = mem.read_u32(0x8c236e28).unwrap_or(0x0000_eeee);
    } else {
        cpu.r[3] = mem.read_u32(0x8c236e2c).unwrap_or(0x0200_0000); // mov.l 0x8c236e2c,r3
        cpu.set_t(cpu.r[13] & cpu.r[3] == 0); // tst r13,r3
        let mode_bit = cpu.r[13] & cpu.r[3] != 0;
        cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bt.s delay: mov.l @r15,r5
        if mode_bit {
            cpu.r[3] = mem.read_u32(0x8c236e30).unwrap_or(0x0100_0000); // mov.l
            cpu.set_t(cpu.r[13] & cpu.r[3] == 0); // tst r13,r3
            let level_bit = cpu.r[13] & cpu.r[3] != 0;
            cpu.r[14] = cpu.r[14].wrapping_add(1); // bt.s delay slot: add #1,r14
            if level_bit {
                cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
            }
            cpu.r[3] = mem.read_u32(0x8c236e34).unwrap_or(0x0040_0000); // mov.l
            cpu.set_t(cpu.r[3] & cpu.r[13] == 0); // tst r3,r13
            if cpu.r[3] & cpu.r[13] == 0 {
                cpu.r[14] = cpu.r[14].wrapping_add(1); // bf not taken: add #1,r14
            }
        }
        // 0x8c236dd6
        cpu.r[0] = mem.read_u16(0x8c236e20).unwrap_or(0x0088) as i16 as i32 as u32; // mov.w
        cpu.r[1] = 7; // mov #7,r1
        cpu.r[3] = mem.read_u32(0x8c236e38).unwrap_or(0x00ff_ffff); // mov.l
        cpu.r[14] &= cpu.r[1]; // and r1,r14 (no T)
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0); // mov.l @(r0,r5),r4
        cpu.r[14] <<= 16; // shll16 r14
        // shll8 r14 is the last T writer; r14 is 0..7 here, so its bit 24 is 0.
        cpu.set_t(cpu.r[14] >> 24 & 1 != 0);
        cpu.r[14] <<= 8; // shll8 r14
        cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r2
        cpu.r[0] = 0; // mov #0,r0
        cpu.r[2] &= cpu.r[3]; // and r3,r2 (no T)
        cpu.r[2] |= cpu.r[14]; // or r14,r2 (no T)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[2]); // mov.l r2,@(8,r4)
    }

    // add #4,r15 ; lds.l @r15+,pr ; mov.l @r15+,r13 ;
    // rts ; delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
