//! 0x8c0d50e4 — load_float_via_field_24.
//!
//! `r0=[r4] ; r0+=24 ; rts ; fr0=[r0]` — a float reached through the pointer
//! stored at `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // delay slot of `rts`
}
