//! 0x8c0102cc — tail-call thunk to `free` (0x8c159fc0).
//!
//! `mov.l @(0x8c0102d4,pc),r0 ; jmp @r0 ; nop`. `jmp` leaves `pr` alone, so the
//! callee's `rts` returns to our caller: `r4` and the callee's whole register
//! residue pass through unchanged. The loaded literal survives the callee's
//! null path, so that path returns 0x8c159fc0 in `r0`; every other path
//! overwrites `r0` inside the callee.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0102d4).unwrap_or(0); // mov.l @(0x8c0102d4,pc),r0
    crate::registry::p_8c159fc0::run(cpu, mem); // jmp @r0 ; nop
}
