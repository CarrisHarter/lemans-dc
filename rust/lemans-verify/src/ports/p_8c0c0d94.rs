//! 0x8c0c0d94 — float_is_zero_via_global.
//!
//! `r0=[0x8c0c0da4] ; r0=[r0] ; r0+=28 ; fr0=[r0] ; fcmp/eq fr1,fr0` with
//! `fr1` cleared — T is 1 exactly when the float is zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_0DA4).unwrap_or(0);
    cpu.fr[1] = 0.0;
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    let zero = cpu.fr[1] == cpu.fr[0];
    cpu.set_t(zero); // fcmp/eq fr1,fr0
}
