//! 0x8c18e1e4 — tail thunk: r4 = *(r4+4), then jmp to 0x8c1c3a2c.
//!
//! The callee pointer is the pool word at 0x8c18e1ec; `jmp` leaves pr untouched, so
//! the callee's `rts` returns to our caller and the delay slot reloads the
//! argument from the first word of the structure.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c18e1ec).unwrap_or(0x8c1c3a2c); // mov.l 0x8c18e1ec,r0
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // delay: mov.l @(4,r4),r4
    crate::registry::p_8c1c3a2c::run(cpu, mem); // jmp @r0
}
