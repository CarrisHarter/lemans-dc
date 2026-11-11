//! 0x8c01acf4 — `return (*(u32*)(base[0x3c] + idx*4))[1];`
//!
//! Same shape as 0x8c01ace8; only the rts delay slot differs:
//! `mov.l @(4,r0),r0` instead of `@(28,r0)`. r1 keeps the table base and
//! r5 is left shifted left by 2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let tbl = mem.read_u32(base + 0x3c).unwrap_or(0);
    cpu.r[1] = tbl;
    cpu.r[5] <<= 2; // shll2
    let entry = mem.read_u32(tbl.wrapping_add(cpu.r[5])).unwrap_or(0);
    cpu.r[0] = entry;
    // delay slot of `rts`
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0);
}
