//! 0x8c231b00 - tail jump to 0x8c239820(1, r4, r5).
//!
//! The `jmp` delay slot undoes `add #-8,r15`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved4 = cpu.r[4];
    let saved5 = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[6] = saved5;
    cpu.r[3] = mem.read_u32(0x8c231b20).unwrap_or(0x8c239820);
    let _ = mem.write_u32(cpu.r[15], saved4);
    cpu.r[4] = 1;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), saved5);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    crate::registry::p_8c239820::run(cpu, mem);
}
