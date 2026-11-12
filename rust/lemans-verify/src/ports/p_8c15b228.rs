//! 0x8c15b228 — tail-call thunk through a two-level pointer.
//!
//! ```text
//! mov.l @r4,r1          ; r1 = dword at r4
//! mov.l 0x8c15b230,r0   ; r0 = 0x8c1c3a2c
//! jmp @r0               ; tail call; pr is NOT touched
//! mov.l @(4,r1),r4      ; delay slot: r4 = dword at (r1 + 4)
//! ```
//!
//! The callee 0x8c1c3a2c stores the float at `r4 + 12` to `(*r4) + 8` and
//! advances r4 by 12. It overwrites r0 on entry, so the loaded literal only
//! determines the jump target; the only residues of this thunk are r1 (the
//! first-level pointer) and the callee's changes to r4/r0/fr0/memory.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = mem.read_u32(0x8C15B230).unwrap_or(0x8C1C3A2C); // mov.l 0x8c15b230,r0
    cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // jmp delay: mov.l @(4,r1),r4
    crate::registry::p_8c1c3a2c::run(cpu, mem); // jmp @r0
}
