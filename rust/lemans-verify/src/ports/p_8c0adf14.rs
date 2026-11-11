//! 0x8c0adf14 — load_through_field.
//!
//! `r0=[r4+4] ; rts ; r0=[r0+12]` — the second load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0); // delay slot of `rts`
}
