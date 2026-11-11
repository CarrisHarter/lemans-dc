//! 0x8c0c1744 — load_field_4_via_36.
//!
//! `r0=[r4+36] ; rts ; r0=[r0+4]` — the load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0); // delay slot of `rts`
}
