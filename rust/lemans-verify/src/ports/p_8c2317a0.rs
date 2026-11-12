//! 0x8c2317a0 - tail jump to 0x8c239820(2, r4, r5).
//!
//! Same frame as 0x8c2315e0; the immediate in r4 is 2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved4 = cpu.r[4];
    let saved5 = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[6] = saved5;
    cpu.r[3] = mem.read_u32(0x8c2317c0).unwrap_or(0x8c239820);
    let _ = mem.write_u32(cpu.r[15], saved4);
    cpu.r[4] = 2;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), saved5);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    crate::registry::p_8c239820::run(cpu, mem);
}
