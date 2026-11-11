//! 0x8c0612f8 — set the SR IMASK nibble to `r4 & 15`, preserving other bits.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[0] = cpu.r[4];
    cpu.r[1] = mem.read_u32(0x8c061310).unwrap_or(0); // 0xffffff0f
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] &= cpu.r[1];
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] |= cpu.r[0];
    cpu.sr = cpu.r[3]; // ldc r3,sr
}
