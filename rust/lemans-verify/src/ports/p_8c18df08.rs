//! 0x8c18df08 — `mov.l 0x8c18df10,r0 ; jmp @r0 ; mov.l @(4,r4),r4`.
//!
//! Load the callee address from the literal pool, then tail-call it. The delay
//! slot reloads r4 from the word at r4+4, so the callee sees the nested
//! pointer; 0x8c1c3a2c dereferences it and stores through the result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C18DF10).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // jmp delay slot
    crate::registry::p_8c1c3a2c::run(cpu, mem); // jmp @r0
}
