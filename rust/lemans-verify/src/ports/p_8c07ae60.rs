//! 0x8c07ae60 — store_three_args_2.
//!
//! `[r4]=r5 ; r0=r4 ; [r4+4]=r6 ; rts ; [r4+8]=r7` — the third store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let _ = mem.write_u32(base, cpu.r[5]);
    cpu.r[0] = base;
    let _ = mem.write_u32(base + 4, cpu.r[6]);
    let _ = mem.write_u32(base + 8, cpu.r[7]); // delay slot of `rts`
}
