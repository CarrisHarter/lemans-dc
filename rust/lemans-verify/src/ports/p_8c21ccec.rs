//! 0x8c21ccec — indexed_word_from_table.
//!
//! `r0 = 4*r4 ; rts ; r0 = [0x8c2bab24 + r0]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c21cdd4).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0); // delay slot of `rts`
}
