//! 0x8c10b7a4 — strcpy(dst = r4 + 0x23c, src = r5) via a tail jump.
//!
//! `r0 = [0x8c10b7b0]` (0x23c), `r4 += r0`, then `jmp` to the pool word at
//! 0x8c10b7b4 (0x8c017f48) with a nop delay slot. `pr` is untouched, so the
//! callee's `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c10b7b0).unwrap_or(0x23c); // mov.l 0x8c10b7b0,r0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
    cpu.r[0] = mem.read_u32(0x8c10b7b4).unwrap_or(0x8c017f48); // mov.l 0x8c10b7b4,r0
    crate::registry::p_8c017f48::run(cpu, mem); // jmp @r0 ; nop
}
