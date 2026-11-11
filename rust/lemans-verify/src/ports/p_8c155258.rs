//! 0x8c155258 — store_r5_at_40.
//!
//! `rts ; [r4+40]=r5` — a single field store in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 40, cpu.r[5]); // delay slot of `rts`
}
