//! 0x8c0c814c — warm the LFSR table, then hash the caller's buffer with it.
//!
//! Saves r14/r13/r12; calls 0x8c0c81ec with the incoming r4/r5/r6 (the warm-up
//! ignores them), then calls 0x8c0c817c with the *saved* (r4,r5,r6) — the
//! `mov r12,r4` lands in the jsr delay slot. All three saved registers are
//! pushed, so they end at entry values; the residue is the hash callee's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved4 = cpu.r[4]; // mov r4,r12
    let saved5 = cpu.r[5]; // mov r5,r13
    let saved6 = cpu.r[6]; // mov r6,r14
    // r12/r13/r14 are pushed, so they end at their ENTRY values; the saved
    // argument copies only live in local variables.

    cpu.r[0] = mem.read_u32(0x8c0c8174).unwrap_or(0x8c0c81ec); // mov.l 0x8c0c8174,r0
    cpu.pr = 0x8c0c8160;
    crate::registry::p_8c0c81ec::run(cpu, mem); // jsr @r0 ; nop

    cpu.r[0] = mem.read_u32(0x8c0c8178).unwrap_or(0x8c0c817c); // mov.l 0x8c0c8178,r0
    cpu.r[6] = saved6; // mov r14,r6
    cpu.r[5] = saved5; // mov r13,r5
    cpu.r[4] = saved4; // delay slot: mov r12,r4
    cpu.pr = 0x8c0c816a;
    crate::registry::p_8c0c817c::run(cpu, mem); // jsr @r0
}
