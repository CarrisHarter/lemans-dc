//! 0x8c018fa4 — `thunk_FUN_8c15a27c`.
//!
//! `mov.l 0x8c018fac,r0 ; jmp @r0 ; nop` — load the callee entry from the
//! literal pool and tail-jump to it. The pool word in the shipped image is
//! 0x8c15a27c. `jmp` leaves `pr` alone, so the callee's `rts` returns straight
//! to our caller, and the delay-slot `nop` changes nothing. The callee
//! overwrites `r0` with the doubly indirected global, so the loaded literal is
//! not part of the register residue; `r4`-`r15`, `sr` and memory are untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C018FAC).unwrap_or(0); // mov.l @(0x8c018fac,pc),r0
    crate::registry::p_8c15a27c::run(cpu, mem); // jmp @r0 ; nop
}
