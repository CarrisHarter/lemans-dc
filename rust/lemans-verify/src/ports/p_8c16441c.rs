//! 0x8c16441c — store_r5_at_20.
//!
//! `rts ; [r4+20]=r5` — a single field store in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 20, cpu.r[5]); // delay slot of `rts`
}
