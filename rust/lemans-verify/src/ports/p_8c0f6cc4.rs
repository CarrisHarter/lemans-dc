//! 0x8c0f6cc4 — advance_68_load.
//!
//! `add #68,r4 ; rts ; mov.l @r4,r0` — `r4` advanced before the delay-slot load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(68);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // delay slot of `rts`
}
