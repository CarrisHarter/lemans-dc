//! 0x8c0f76b0 — advance_56_load_float_2.
//!
//! `add #56,r4 ; rts ; fmov @r4,fr0` — `r4` advanced before the delay-slot load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(56);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // delay slot of `rts`
}
