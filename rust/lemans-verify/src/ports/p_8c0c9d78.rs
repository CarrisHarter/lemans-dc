//! 0x8c0c9d78 - tail jump to 0x8c1031e0 with `r4 = [r4+4]`.
//!
//! `jmp` leaves pr alone, so the callee's `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0c9d80).unwrap_or(0x8c1031e0);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // jmp delay
    crate::registry::p_8c1031e0::run(cpu, mem);
}
