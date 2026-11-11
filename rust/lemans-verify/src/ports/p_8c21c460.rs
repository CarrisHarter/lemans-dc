//! 0x8c21c460 — restore the SR IMASK nibble from `*r4 & 15` (companion of 0x8c21c448).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[2] = cpu.sr; // stc sr,r2
    cpu.r[1] = mem.read_u16(0x8c21c572).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[2] &= cpu.r[1];
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] |= cpu.r[2];
    cpu.sr = cpu.r[0]; // ldc r0,sr
}
