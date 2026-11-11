//! 0x8c07cdc8 — load_pool_constant_2.
//!
//! `mov.l 0x8c07cdd0,r0 ! 8c1d18a8 ; rts ; nop` — r0 is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C07_CDD0).unwrap_or(0);
}
