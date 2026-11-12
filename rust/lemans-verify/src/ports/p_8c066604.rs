//! 0x8c066604 — tail-jump into 0x8c067f08 with `[r4+0x38]` as the new `r4`.
//!
//! Original: `mov.l 0x8c06660c,r0 ; jmp @r0 ; mov.l @(56,r4),r4`. The literal
//! holds 0x8c067f08. `jmp` leaves `pr` alone, so the callee's `rts` returns to
//! the caller's return address; the delay slot reloads `r4` from the argument
//! block before the tail call. r0 is left holding the code pointer only until
//! the callee overwrites it with 0x9c.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C06660C).unwrap(); // mov.l, literal = 0x8c067f08
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap(); // delay slot of `jmp`
    crate::registry::p_8c067f08::run(cpu, mem); // jmp @r0
}
