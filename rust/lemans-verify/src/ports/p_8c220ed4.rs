//! 0x8c220ed4 — tail thunk: jmp to 0x8c221d54 (pool word at 0x8c22102c).
//!
//! `jmp` leaves pr untouched, so the callee's `rts` returns to our caller; the
//! delay slot is a nop and r3 keeps the callee pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c22102c).unwrap_or(0x8c221d54); // mov.l 0x8c22102c,r3
    crate::registry::p_8c221d54::run(cpu, mem); // jmp @r3 ; nop
}
