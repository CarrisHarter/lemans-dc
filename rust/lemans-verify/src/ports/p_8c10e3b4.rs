//! 0x8c10e3b4 — load_field_plus_20_2.
//!
//! `r0=[r4] ; rts ; r0+=20` — the pointer plus 20.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(20); // delay slot of `rts`
}
