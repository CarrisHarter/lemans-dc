//! 0x8c0b2c00 — hypot.
//!
//! `fr0=[r4+] ; fr1=[r4] ; fr0*=fr0 ; fr1*=fr1 ; fr0+=fr1 ; rts ; fsqrt fr0` —
//! the length of a 2-vector, with `r4` advanced by 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[0];
    cpu.fr[1] = cpu.fr[1] * cpu.fr[1];
    cpu.fr[0] = cpu.fr[0] + cpu.fr[1];
    cpu.fr[0] = cpu.fr[0].sqrt(); // delay slot of `rts`
}
