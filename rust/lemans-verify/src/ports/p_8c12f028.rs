//! 0x8c12f028 — load_pool_constant_7.
//!
//! `mov.l 0x8C12_F030,r0 ; rts ; nop` — `r0` is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C12_F030).unwrap_or(0);
}
