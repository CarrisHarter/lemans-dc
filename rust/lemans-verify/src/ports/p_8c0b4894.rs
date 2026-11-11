//! 0x8c0b4894 — advance_32_load_float.
//!
//! `add #32,r4 ; rts ; fmov @r4,fr0` — `r4` is advanced before the delay-slot load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(32);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // delay slot of `rts`
}
