//! 0x8c1570a0 — store_r5_at_56.
//!
//! `rts ; [r4+56]=r5` — a single field store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 56, cpu.r[5]); // delay slot of `rts`
}
