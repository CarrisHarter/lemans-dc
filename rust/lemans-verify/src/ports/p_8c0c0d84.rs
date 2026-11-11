//! 0x8c0c0d84 — load_float_via_global_28.
//!
//! `r0=[0x8C0C_0D90] ; r0=[r0] ; r0+=28 ; rts ; fr0=[r0]` — a float reached
//! through a global pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_0D90).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // delay slot of `rts`
}
