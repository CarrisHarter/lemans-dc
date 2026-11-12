//! 0x8c07de4c — `mov.l 0x8c07de54,r0 ; jmp @r0 ; mov.l @(4,r4),r4`.
//!
//! Tail jump to 0x8c079064 (thunk to FUN_8c05fa50), the literal at 0x8c07de54.
//! `jmp` leaves `pr` alone, so the callee's `rts` returns straight to our
//! caller. The delay slot reloads `r4` from its own argument block before the
//! jump; that is the only register effect that survives, since the callee
//! overwrites `r0` with its own literal load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c07de54).unwrap_or(0); // mov.l 0x8c07de54,r0 ! 8c079064
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // jmp delay slot
    crate::registry::p_8c079064::run(cpu, mem);
}
