//! 0x8c0c8f18 — load_field_56.
//!
//! `rts ; r0=[r4+56]` — the load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 56).unwrap_or(0); // delay slot of `rts`
}
