//! 0x8c01ace8 — `return (*(u32*)(base[0x3c] + idx*4))[0x1c];`
//!
//! `mov.l @(60,r4),r1 ; shll2 r5 ; mov r5,r0 ; mov.l @(r0,r1),r0 ; rts
//!  ; mov.l @(28,r0),r0`
//! Hidden side effects: r1 keeps the table base, r5 is left shifted left by 2
//! (shll2 does not touch T), and the rts delay slot performs the final load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let tbl = mem.read_u32(base + 0x3c).unwrap_or(0);
    cpu.r[1] = tbl;
    cpu.r[5] <<= 2; // shll2
    let entry = mem.read_u32(tbl.wrapping_add(cpu.r[5])).unwrap_or(0);
    cpu.r[0] = entry;
    // delay slot of `rts`
    cpu.r[0] = mem.read_u32(cpu.r[0] + 0x1c).unwrap_or(0);
}
