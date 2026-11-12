//! 0x8c2199d8 - thin call wrapper around 0x8c21aff2.
//!
//! Pushes pr, calls the callee with the caller's registers untouched, and
//! returns its r0. Nothing else is observable.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c219a18).unwrap_or(0x8c21aff2); // mov.l 0x8c219a18,r3
    cpu.pr = 0x8c2199e0; // jsr delay is nop
    crate::registry::p_8c21aff2::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
