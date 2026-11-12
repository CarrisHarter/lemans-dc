//! 0x8c21afd0 - pointer to slot `r4` in a 44-byte table at `*[0x8c2ba634]+40`.
//!
//! `mul.l` is signed; `macl` is the low word and `mach` the high word. The
//! return is the `rts` delay slot `add r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 44; // mov #44,r2
    cpu.r[3] = mem.read_u32(0x8c21b018).unwrap_or(0x8c2ba634); // mov.l 0x8c21b018,r3
    let product = (cpu.r[2] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r2,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r0
    cpu.r[0] = cpu.r[0].wrapping_add(40); // add #40,r0
    cpu.r[4] = cpu.macl; // sts macl,r4
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // rts delay: add r4,r0
}
