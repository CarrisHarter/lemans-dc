//! 0x8c231660 - tail jump to 0x8c239820(8, r4, 0x8c2c6f5c).
//!
//! The `jmp` delay slot undoes `add #-4,r15`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved4 = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[5] = saved4;
    cpu.r[3] = mem.read_u32(0x8c231684).unwrap_or(0x8c239820);
    let _ = mem.write_u32(cpu.r[15], saved4);
    cpu.r[4] = 8;
    cpu.r[6] = mem.read_u32(0x8c231680).unwrap_or(0x8c2c6f5c);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    crate::registry::p_8c239820::run(cpu, mem);
}
