//! 0x8c14fe10 — load_via_field_plus_24.
//!
//! `r0=[r4+4] ; r0+=24 ; rts ; r0=[r0+4]` — two levels through the +4 field.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0); // delay slot of `rts`
}
