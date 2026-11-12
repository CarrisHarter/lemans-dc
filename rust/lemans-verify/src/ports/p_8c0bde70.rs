//! 0x8c0bde70 — element lookup: `[r4+16] + 108*r5`.
//!
//! `r0 = 108`, signed `mul.l r0,r5` (setting MACH/MACL), `r5 = macl`, then the
//! `rts` delay slot adds the base pointer loaded from `[r4+16]`. r0 is the
//! element address, r5 the byte offset.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 108; // mov #108,r0
    let product = 108i64 * (cpu.r[5] as i32 as i64); // mul.l r0,r5
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[5] = cpu.macl; // sts macl,r5
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0); // mov.l @(16,r4),r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]); // rts delay: add r5,r0
}
