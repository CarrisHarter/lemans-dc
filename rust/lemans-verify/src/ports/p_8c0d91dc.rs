//! 0x8c0d91dc — reciprocal_at_120.
//!
//! `add #120,r4 ; fr1=[r4] ; fr0=1.0 ; rts ; fr0=fr0/fr1` — the
//! reciprocal of the float at the advanced `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(120);
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[0] = 1.0;
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1]; // delay slot of `rts`
}
