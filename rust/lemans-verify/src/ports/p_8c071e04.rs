//! 0x8c071e04 — load_global_8c2648f0.
//!
//! `mov.l 0x8c071e0c,r0 ! 8c2648f0 ; rts ; mov.l @r0,r0` — the load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C07_1E0C).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
