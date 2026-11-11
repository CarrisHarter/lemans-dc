//! 0x8c0d5bf8 — store_two_args_at_0_4.
//!
//! `[r4]=r5 ; rts ; [r4+4]=r6` — the second store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let _ = mem.write_u32(base, cpu.r[5]);
    let _ = mem.write_u32(base + 4, cpu.r[6]); // delay slot of `rts`
}
