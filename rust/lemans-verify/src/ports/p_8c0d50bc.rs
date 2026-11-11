//! 0x8c0d50bc — load_field_8.
//!
//! `rts ; mov.l @(8,r4),r0` — the load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0); // delay slot of `rts`
}
