//! 0x8c21aff2 - pointer to slot `r4` in an 8-byte table at `*[0x8c2ba634]+4`.
//!
//! `shll r4` is the only T writer: T is bit 29 of the incoming index.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b018).unwrap_or(0x8c2ba634); // mov.l 0x8c21b018,r3
    cpu.r[4] = cpu.r[4].wrapping_shl(2); // shll2 r4
    cpu.set_t(cpu.r[4] >> 31 != 0); // shll r4 sets T from bit 31
    cpu.r[4] = cpu.r[4].wrapping_shl(1); // shll r4
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r0
    cpu.r[0] = cpu.r[0].wrapping_add(4); // add #4,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // rts delay: add r4,r0
}
