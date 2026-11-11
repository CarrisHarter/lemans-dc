//! 0x8c2199b8 — index_times_24.
//!
//! `r0 = 24*r4 + [0x8c219a0c]` via shifts.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4];
    cpu.r[0] = cpu.r[0] << 1;
    cpu.r[3] = cpu.r[4];
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.r[2] = mem.read_u32(0x8c219a0c).unwrap_or(0);
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[0] = cpu.r[0] << 1;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]); // delay slot of `rts`
}
