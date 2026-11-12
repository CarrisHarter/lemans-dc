//! 0x8c17390c — file `r4` into the 0x8c2b5260 table, bump its mod-64
//! counter, and set SR IMASK from the 0x8c1739f4(15) result.
//!
//! Original: spills `r4`, calls 0x8c1739f4 with `r4=15` (delay slot), stores
//! the spilled `r4` at `table+0xb4+[table+0xb0]*4`, increments
//! `[table+0xb0]` mod 64, then `SR = (SR & mask) | ((r5 & 15) << 4)` where
//! the `mov.w` mask literal 0xff0f sign-extends to 0xffffff0f. The stack
//! spill is invisible (stack window excluded), so `r15` is left untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c1739b8).unwrap_or(0);
    let saved_r4 = cpu.r[4];
    cpu.r[4] = 15; // delay slot of `jsr`
    crate::registry::p_8c1739f4::run(cpu, mem);
    cpu.r[5] = cpu.r[0]; // mov r0,r5: callee result also lives in r5
    let r5 = cpu.r[5];
    cpu.r[0] = mem.read_u16(0x8c1739ae).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[4] = mem.read_u32(0x8c1739bc).unwrap_or(0);
    cpu.r[3] = mem.read_u16(0x8c1739b0).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[1] = saved_r4; // mov.l @r15,r1: the spill the oracle pushes
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    cpu.r[0] = mem.read_u16(0x8c1739ae).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[2] = cpu.r[2].wrapping_shl(2);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
    let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[1] = mem.read_u16(0x8c1739ae).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] = cpu.r[0].wrapping_add(1);
    cpu.r[2] = mem.read_u16(0x8c1739b2).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[4]);
    cpu.r[0] &= 63;
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.r[0] = r5;
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[0] &= 15;
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
}
