//! 0x8c165918 — store_r5_at_0.
//!
//! `rts ; [r4]=r5` — a single word store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]); // delay slot of `rts`
}
