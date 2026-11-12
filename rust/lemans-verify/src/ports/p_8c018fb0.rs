//! 0x8c018fb0 — `thunk_FUN_8c15a23c`.
//!
//! `mov.l 0x8c018fb8,r0 ; jmp @r0 ; nop` — load the code pointer from the
//! literal pool and tail-call it. `jmp` leaves `pr` alone, so the callee's
//! `rts` returns straight to our caller; there is no stack frame and the delay
//! slot is a nop, so every register except the callee's residue passes through
//! untouched. The callee (`find_block_by_size`) writes `r0` on every path, so
//! the loaded pointer itself never survives into the final state.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c018fb8).unwrap_or(0); // mov.l 0x8c018fb8,r0
    crate::registry::p_8c15a23c::run(cpu, mem); // jmp @r0 ; nop
}
