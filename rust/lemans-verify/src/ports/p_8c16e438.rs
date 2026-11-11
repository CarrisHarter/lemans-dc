//! 0x8c16e438 — float_through_two_fields.
//!
//! `add #92,r4 ; r0=[r4] ; r0+=44 ; rts ; fr0=[r0]` — a float two levels down.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(44);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // delay slot of `rts`
}
