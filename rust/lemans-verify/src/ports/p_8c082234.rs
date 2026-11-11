//! 0x8c082234 — store_at_32.
//!
//! `rts ; mov.l r5,@(32,r4)` — the only work is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 32, cpu.r[5]); // delay slot of `rts`
}
