//! 0x8c07fc24 — store_two_args.
//!
//! `[r4+4]=r5 ; rts ; [r4+8]=r6` — the second store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let _ = mem.write_u32(base + 4, cpu.r[5]);
    let _ = mem.write_u32(base + 8, cpu.r[6]); // delay slot of `rts`
}
