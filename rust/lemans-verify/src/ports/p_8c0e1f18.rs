//! 0x8c0e1f18 — advance a descriptor, then initialise its record.
//!
//! `r0 = 0x8c0eaef8(r5)`; then `0x8c227780(r4, 0, r0)`. Saves and restores
//! `r14` and `pr`; both callees run through their ports, so the wrapper is
//! tested against the oracle's real callee bytes.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u32(0x8c0e1f34).unwrap_or(0);
    cpu.r[14] = cpu.r[4];
    cpu.r[4] = cpu.r[5]; // delay slot of `jsr`
    crate::registry::p_8c0eaef8::run(cpu, mem);
    cpu.r[6] = cpu.r[0];
    cpu.r[0] = mem.read_u32(0x8c0e1f38).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.r[5] = 0; // delay slot of `jsr`
    crate::registry::p_8c227780::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
