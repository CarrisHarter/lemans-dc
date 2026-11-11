//! 0x8c0f1ca8 — store_field_20.
//!
//! `rts ; mov.l r5,@(20,r4)` — the only work is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 20, cpu.r[5]); // delay slot of `rts`
}
