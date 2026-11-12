//! 0x8c220eda — tail thunk: jmp to 0x8c221d70 (pool word at 0x8c221030).
//!
//! `jmp` leaves pr untouched, so the callee's `rts` returns to our caller; the
//! delay slot is a nop and r3 keeps the callee pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c221030).unwrap_or(0x8c221d70); // mov.l 0x8c221030,r3
    crate::registry::p_8c221d70::run(cpu, mem); // jmp @r3 ; nop
}
