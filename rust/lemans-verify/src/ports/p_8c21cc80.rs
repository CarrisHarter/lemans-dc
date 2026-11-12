//! 0x8c21cc80 - `0x8c2ba644 + 52*r4`.
//!
//! `mul.l` is signed and writes both `mach` and `macl`. The add is the `rts`
//! delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 52; // mov #52,r0
    cpu.r[3] = mem.read_u32(0x8c21cc94).unwrap_or(0x8c2ba644); // mov.l 0x8c21cc94,r3
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r0,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[0] = cpu.macl; // sts macl,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // rts delay: add r3,r0
}
