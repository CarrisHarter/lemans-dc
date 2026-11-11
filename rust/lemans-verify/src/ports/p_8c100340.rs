//! 0x8c100340 — load_float_indexed.
//!
//! `r0=[0x8c10034c] ; r4<<=2 ; r4+=r0 ; rts ; fmov @r4,fr0` — `r4` becomes the
//! table address and `fr0` is loaded from it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C10_034C).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // delay slot of `rts`
}
