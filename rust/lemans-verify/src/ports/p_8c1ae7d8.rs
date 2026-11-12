//! 0x8c1ae7d8 - zero three fields and publish the object to a global.
//!
//! 0x8c01e484(r4, r5 = r15) runs first (it only echoes r4), then `[r14]`,
//! `[r14+4]` and `[r14+8]` are cleared and `[0x8c2b8280] = r14`. Returns r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c1ae7fc).unwrap_or(0x8c01e484); // mov.l 0x8c1ae7fc,r0
    cpu.r[5] = cpu.r[15]; // jsr delay: mov r15,r5
    cpu.pr = 0x8c1ae7e6;
    crate::registry::p_8c01e484::run(cpu, mem);
    cpu.r[3] = 0; // mov #0,r3
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]); // mov.l r3,@r14
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[1] = mem.read_u32(0x8c1ae800).unwrap_or(0x8c2b8280); // mov.l 0x8c1ae800,r1
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]); // mov.l r3,@(4,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[3]); // mov.l r3,@(8,r14)
    let _ = mem.write_u32(cpu.r[1], cpu.r[14]); // mov.l r14,@r1
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
