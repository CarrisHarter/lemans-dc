//! 0x8c082238 — store_at_28.
//!
//! `rts ; mov.l r5,@(28,r4)` — the only work is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 28, cpu.r[5]); // delay slot of `rts`
}
