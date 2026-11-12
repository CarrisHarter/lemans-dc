//! 0x8c026e70 — tail-call `set([r4+0x2c], 1)`.
//!
//! ```text
//! mov.l @(44,r4),r4   ; r4 = [r4+0x2c]
//! mov.l 0x8c026e78,r0 ; r0 = 0x8c050c58
//! jmp @r0
//! mov #1,r5           ; delay slot
//! ```
//! `jmp` leaves `pr` alone, so the callee's `rts` returns to our caller.
//! Residue: r0 keeps the loaded code pointer, r4 the field, r5 the 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(44)).unwrap_or(0); // mov.l @(44,r4),r4
    cpu.r[0] = mem.read_u32(0x8c026e78).unwrap_or(0); // mov.l 0x8c026e78,r0
    cpu.r[5] = 1; // delay slot of `jmp`
    crate::registry::p_8c050c58::run(cpu, mem); // jmp @r0
}
