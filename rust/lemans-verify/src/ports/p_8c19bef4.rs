//! 0x8c19bef4 - search, then install vtable 0x8c1f1480.
//!
//! Same call as 0x8c19be80 (`fr4 = 0`) but only the byte at +24 is cleared.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.fr[4] = 0.0;
    cpu.pr = 0x8c19bf00;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c19bf18).unwrap_or(0);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    cpu.r[0] = 24;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), 0);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
