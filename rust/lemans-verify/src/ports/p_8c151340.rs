//! 0x8c151340 - tail thunk: `r4 += 32`, then `jmp` to 0x8c0b4f34.
//!
//! `jmp` leaves pr alone and the delay slot adds 32 to the argument, so the
//! callee's `rts` returns straight to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c151348).unwrap_or(0x8c0b4f34); // mov.l 0x8c151348,r0
    cpu.r[4] = cpu.r[4].wrapping_add(32); // jmp delay: add #32,r4
    crate::registry::p_8c0b4f34::run(cpu, mem);
}
