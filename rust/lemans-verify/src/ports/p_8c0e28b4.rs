//! 0x8c0e28b4 — load `FUN_8c0e8e08` and tail-jump with `r5 = 7`.
//!
//! `mov.l 0x8c0e28bc,r0 ; jmp @r0 ; mov #7,r5` — the literal pool word is
//! 0x8c0e8e08, the delay slot loads the slot index 7 (second argument) and
//! `jmp` leaves `pr` alone, so the callee's `rts` returns straight to our
//! caller. The callee immediately overwrites `r0` with `r4`, so the loaded
//! literal is not part of the register residue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0E28BC).unwrap_or(0); // mov.l @(0x8c0e28bc,pc),r0
    cpu.r[5] = 7; // delay slot of `jmp @r0`: mov #7,r5
    crate::registry::p_8c0e8e08::run(cpu, mem); // jmp @r0
}
