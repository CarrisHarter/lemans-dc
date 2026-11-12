//! 0x8c07de58 — `FUN_8c07de58`.
//!
//! `mov.l 0x8c07de60,r0 ; jmp @r0 ; mov.l @(4,r4),r4` — load the code pointer
//! 0x8c079078 and tail-jump to it; the delay slot reloads `r4` from the word
//! at `r4 + 4`. The callee (`add #68,r4 ; rts ; mov.l @r4,r0`) then reads the
//! value at that pointer plus 0x44 into `r0`, overwriting the loaded pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Delay slot of `jmp @r0`: load after the jump is taken.
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    crate::registry::p_8c079078::run(cpu, mem); // jmp @r0
}
