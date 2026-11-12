//! 0x8c0c0100 — tail-call thunk to `advance_56` (0x8c0f816c).
//!
//! `mov.l 0x8c0c0108,r0 ; jmp @r0 ; mov.l @(4,r4),r4`. The delay slot
//! reloads the argument from `[r4+4]` before the tail jump; `jmp` leaves `pr`
//! alone, so the callee's `rts` returns to our caller. The callee then does
//! `r4 += 56` and its own delay slot copies the result to `r0`, overwriting the
//! loaded pointer — that sum is the whole residue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C0108).unwrap(); // mov.l 0x8c0c0108,r0
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // delay slot of `jmp`
    crate::registry::p_8c0f816c::run(cpu, mem); // jmp @r0
}
