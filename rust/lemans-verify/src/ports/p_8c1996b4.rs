//! 0x8c1996b4 - indexed field address: `r0 = [r4+32] + low32(0xe4 * r5)`.
//!
//! `mul.l` (signed) writes MACH:MACL and only MACL is read back, so a negative
//! `r5` contributes its low product word. The `rts` delay slot does the add.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1996c0).unwrap_or(0xe4); // mov.l 0x8c1996c0,r0
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[5] as i32 as i64); // mul.l r0,r5
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[5] = cpu.macl; // sts macl,r5
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0); // mov.l @(32,r4),r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]); // rts delay: add r5,r0
}
