//! 0x8c0e0694 — reload a published object and tail-jump to its handler.
//!
//! `r0 = [0x8c0e06a0]` (0x8c27a930), `r4 = *r0`, then `jmp` to the pool word at
//! 0x8c0e06a4. `pr` is untouched, so the callee's `rts` returns to the caller;
//! the delay slot is a nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0e06a0).unwrap_or(0x8c27a930); // mov.l 0x8c0e06a0,r0
    cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r4
    cpu.r[0] = mem.read_u32(0x8c0e06a4).unwrap_or(0x8c1bfb20); // mov.l 0x8c0e06a4,r0
    // jmp @r0 with a nop delay slot: run the callee in place (jr-style, so the
    // callee's rts returns to our caller and pr is untouched)
    crate::registry::p_8c1bfb20::run(cpu, mem);
}
