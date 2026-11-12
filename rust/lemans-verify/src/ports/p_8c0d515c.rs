//! 0x8c0d515c — tail call: `callee(*(u32 *)(r4 + 8))`.
//!
//! ```text
//! mov.l 0x8c0d5164,r0    ; r0 = 0x8c06caa8
//! jmp   @r0              ; tail call; pr is NOT touched, it belongs to the caller
//! mov.l @(8,r4),r4       ; delay slot: the argument is the word at [r4+8]
//! ```
//!
//! The thunk contributes only `r0` (the loaded pointer) and
//! `r4 = *(u32 *)(r4 + 8)`; every other register, T and all stores belong to
//! the callee, which also overwrites `r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0D5164).unwrap_or(0x8c06caa8); // mov.l @(0x8c0d5164,pc),r0
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // delay slot of `jmp`
    crate::registry::p_8c06caa8::run(cpu, mem); // jmp @r0
}
