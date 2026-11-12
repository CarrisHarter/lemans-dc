//! 0x8c0d5150 — tail-call 0x8c06caa8 with the word at `r4+4`.
//!
//! `mov.l 0x8c0d5158,r0 ; jmp @r0 ; mov.l @(4,r4),r4`
//! The pooled literal is 0x8c06caa8; the delay slot replaces `r4` with the word
//! it points past, and the callee advances that by 12 into both `r4` and `r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0D5158).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // delay slot of `jmp`
    crate::registry::p_8c06caa8::run(cpu, mem); // jmp @r0
}
