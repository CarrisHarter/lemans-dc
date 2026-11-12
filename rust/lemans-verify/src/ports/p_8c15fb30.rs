//! 0x8c15fb30 - double-indirect tail thunk into 0x8c1c3a2c.
//!
//! The `jmp` delay slot loads `r4 = [[r4]+4]` before the callee runs; pr is
//! untouched, so the callee returns straight to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = mem.read_u32(0x8c15fb38).unwrap_or(0x8c1c3a2c); // mov.l 0x8c15fb38,r0
    cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // jmp delay: mov.l @(4,r1),r4
    crate::registry::p_8c1c3a2c::run(cpu, mem);
}
