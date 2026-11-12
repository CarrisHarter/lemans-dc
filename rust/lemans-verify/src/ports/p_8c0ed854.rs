//! 0x8c0ed854 — zero a struct then return its pointer.
//!
//! Saves r14/pr, calls 0x8c0ed80c (the 0x50-byte zeroer) with the same r4, then
//! `r0 = r14`. r15 and r14 are restored, so only the callee's r0/r1 residue and
//! the zeroed object are observable.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14]; // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c0ed868).unwrap_or(0x8c0ed80c); // mov.l 0x8c0ed868,r0
    cpu.pr = 0x8c0ed860;
    crate::registry::p_8c0ed80c::run(cpu, mem); // jsr @r0
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[15] = cpu.r[15].wrapping_add(8); // lds.l @r15+,pr ; rts delay: @r15+,r14
    cpu.r[14] = saved14;
}
