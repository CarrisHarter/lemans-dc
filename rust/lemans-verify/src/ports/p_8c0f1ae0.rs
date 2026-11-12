//! 0x8c0f1ae0 — tail thunk: r4 = *r4, then jmp to 0x8c1c3a2c.
//!
//! `jmp` leaves pr untouched; the delay slot reloads the argument from the
//! structure's first word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0f1ae8).unwrap_or(0x8c1c3a2c); // mov.l 0x8c0f1ae8,r0
    cpu.r[4] = mem.read_u32(cpu.r[4]).unwrap_or(0); // delay slot: mov.l @r4,r4
    crate::registry::p_8c1c3a2c::run(cpu, mem); // jmp @r0
}
