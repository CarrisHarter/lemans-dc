//! 0x8c12d30c — call two helpers in sequence.
//!
//! Loads the two target addresses from the literal pool at 0x8c12d320 /
//! 0x8c12d324, calls 0x8c15a27c (double indirect global) then 0x8c15a23c
//! (block-list walk) with `r4` unchanged, saving `pr` around both.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c12d320).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    crate::registry::p_8c15a27c::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c12d324).unwrap_or(0);
    crate::registry::p_8c15a23c::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
