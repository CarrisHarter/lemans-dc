//! 0x8c0c9d84 - tail thunk: `r4 = [r4+4]`, then `jmp` to 0x8c103604.
//!
//! The load is the `jmp` delay slot; pr is untouched, so the callee returns
//! straight to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0c9d8c).unwrap_or(0x8c103604); // mov.l 0x8c0c9d8c,r0
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // jmp delay: mov.l @(4,r4),r4
    crate::registry::p_8c103604::run(cpu, mem);
}
