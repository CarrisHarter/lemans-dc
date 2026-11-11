//! 0x8c21aff2 — index_times_8_plus_field.
//!
//! `r0 = [0x8c2ba634] + 4 + 8*r4` (the index is scaled by two shifts).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b018).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4] << 1;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
