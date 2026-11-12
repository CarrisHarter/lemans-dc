//! 0x8c219cce - look up a record and pack its first four bytes big-endian.
//!
//! 0x8c219c9e(r5, 1) returns a record pointer; null returns -1. Otherwise the
//! word stored at `[r4]` is `(b0<<24) | (b1<<16) | (b2<<8) | b3` and the function
//! returns 0. The `shll16`/`shll8` pair only ever sees a zero high bit, so T ends
//! clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r15)
    cpu.r[5] = 1; // mov #1,r5
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // bsr delay: mov.l @(4,r15),r4
    cpu.pr = 0x8c219cdc;
    crate::registry::p_8c219c9e::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut result;
    if !cpu.t() {
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(1)).unwrap_or(0) as u32; // mov.b @(1,r4),r0
        cpu.r[1] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32; // mov.b @r4,r1
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
        cpu.set_t(cpu.r[0] >> 15 != 0); // shll16 r0
        cpu.r[0] = cpu.r[0].wrapping_shl(16);
        cpu.r[2] = cpu.r[0]; // mov r0,r2
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(2)).unwrap_or(0) as u32; // mov.b @(2,r4),r0
        cpu.r[1] &= 0xff; // extu.b r1,r1
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] >> 23 != 0); // shll8 r0
        cpu.r[0] = cpu.r[0].wrapping_shl(8);
        cpu.r[2] |= cpu.r[0]; // or r0,r2
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(3)).unwrap_or(0) as u32; // mov.b @(3,r4),r0
        cpu.set_t(cpu.r[1] >> 15 != 0); // shll16 r1
        cpu.r[1] = cpu.r[1].wrapping_shl(16);
        cpu.set_t(cpu.r[1] >> 23 != 0); // shll8 r1
        cpu.r[1] = cpu.r[1].wrapping_shl(8);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[2] |= cpu.r[1]; // or r1,r2
        cpu.r[2] |= cpu.r[0]; // or r0,r2
        cpu.r[0] = 0; // mov #0,r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[2]); // mov.l r2,@r3
        result = 0;
    } else {
        result = 0xffff_ffff; // mov #-1,r0
    }
    cpu.r[0] = result;
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
