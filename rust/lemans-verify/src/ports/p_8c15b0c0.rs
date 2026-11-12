//! 0x8c15b0c0 — pointer-chain thunk into `set_mode_byte` (0x8c081fd0).
//!
//! `mov.l @r4,r1` three times walks a chain of pointers; the `jmp @r0` delay
//! slot then loads the fourth word into `r4`, so the callee receives the value
//! four dereferences deep. `jmp` leaves `pr` alone, so the callee's `rts`
//! returns straight to our caller. The chain loads touch no flags; the final
//! flags and register residue are the callee's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = 0x8C081FD0; // mov.l 0x8c15b0cc,r0
    cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r1
    cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r1
    cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay slot: mov.l @r1,r4
    crate::registry::p_8c081fd0::run(cpu, mem); // jmp @r0
}
