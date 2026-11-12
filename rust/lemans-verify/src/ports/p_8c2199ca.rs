//! 0x8c2199ca - call 0x8c21afe0 and return its result.
//!
//! Pushes pr, `jsr` with a nop delay slot, restores pr. Every other register,
//! T and the stores are the callee's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c219a14).unwrap_or(0x8c21afe0); // mov.l 0x8c219a14,r3
    cpu.pr = 0x8c2199d2; // jsr delay is nop
    crate::registry::p_8c21afe0::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
