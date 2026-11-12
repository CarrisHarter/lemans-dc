//! 0x8c21afe0 - pointer to slot `r4` in a 120-byte table at `*[0x8c2ba634]+0x44c`.
//!
//! Same shape as 0x8c21afd0. The displacement is the sign-extended word at
//! 0x8c21b00a (0x044c). `mul.l` writes both `mach` and `macl`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 120; // mov #120,r2
    cpu.r[3] = mem.read_u32(0x8c21b018).unwrap_or(0x8c2ba634); // mov.l 0x8c21b018,r3
    let product = (cpu.r[2] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r2,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[1] = mem.read_u16(0x8c21b00a).unwrap_or(0x044c) as i16 as i32 as u32; // mov.w
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r0
    cpu.r[4] = cpu.macl; // sts macl,r4
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // rts delay: add r4,r0
}
