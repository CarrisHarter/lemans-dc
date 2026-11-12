//! 0x8c079064 — `thunk_FUN_8c05fa50`.
//!
//! `mov.l 0x8c07906c,r0 ; jmp @r0 ; nop` — load the code pointer (0x8c05fa50)
//! from the literal pool and tail-call it. `jmp` leaves `pr` alone, so the
//! callee's `rts` returns straight to our caller; the delay slot does nothing.
//! The callee overwrites r0, so the loaded pointer itself never survives.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c07906c).unwrap_or(0); // mov.l 0x8c07906c,r0
    crate::registry::p_8c05fa50::run(cpu, mem); // jmp @r0 ; nop
}
