//! 0x8c0c6104 — tail thunk into 0x8c05d28c.
//!
//! `r0 = [0x8c0c610c]`, `jmp @r0` with a nop delay slot; `pr` is untouched, so
//! the callee's `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0c610c).unwrap_or(0x8c05d28c); // mov.l 0x8c0c610c,r0
    crate::registry::p_8c05d28c::run(cpu, mem); // jmp @r0 ; nop
}
