//! 0x8c051868 — tail-call dispatch through a per-object pointer.
//!
//! `mov.l @(0x8c051874,pc),r0 ; mov.l @(r0,r4),r4 ; mov.l @(0x8c051878,pc),r0 ;
//! jmp @r0 ; nop`. r0 becomes 0x190, r4 becomes the word at r4+0x190 (the
//! object's record base), then 0x8c02baf8 — the 6-float record copier — runs
//! with that base. `jmp` does not write `pr`, so the callee's `rts` returns
//! straight to our caller; the delay slot is a nop. Everything left in the
//! registers (including r0, which the callee ends as [source+0x4a0]) is the
//! callee's residue. There is no other path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C051874).unwrap_or(0); // 0x190
    cpu.r[4] = mem
        .read_u32(cpu.r[0].wrapping_add(cpu.r[4]))
        .unwrap_or(0); // mov.l @(r0,r4),r4
    cpu.r[0] = mem.read_u32(0x8C051878).unwrap_or(0); // 0x8c02baf8
    // jmp @r0 ; nop — pr untouched, delay slot does nothing.
    crate::registry::p_8c02baf8::run(cpu, mem);
}
