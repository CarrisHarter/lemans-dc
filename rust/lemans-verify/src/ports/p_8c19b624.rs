//! 0x8c19b624 - tail thunk: load `[r4+100]` into fr4, then `jmp` to 0x8c16ee30.
//!
//! `jmp` leaves pr alone, so the callee's `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(100); // mov r4,r0 ; add #100,r0
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr4
    cpu.r[0] = mem.read_u32(0x8c19b630).unwrap_or(0x8c16ee30); // mov.l 0x8c19b630,r0
    crate::registry::p_8c16ee30::run(cpu, mem);
}
