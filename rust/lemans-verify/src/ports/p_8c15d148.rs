//! 0x8c15d148 - tail thunk: `r4 = [[r4]+4]`, then `jmp` to 0x8c1c3a2c.
//!
//! The load of the argument is the `jmp` delay slot, so the new r4 is the
//! callee's r4 before it runs. pr is untouched: the callee returns to us.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = mem.read_u32(0x8c15d150).unwrap_or(0x8c1c3a2c); // mov.l 0x8c15d150,r0
    cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // jmp delay: mov.l @(4,r1),r4
    crate::registry::p_8c1c3a2c::run(cpu, mem);
}
