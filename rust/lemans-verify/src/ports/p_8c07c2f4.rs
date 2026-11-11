//! 0x8c07c2f4 — load_pool_constant.
//!
//! `mov.l 0x8c07c2fc,r0 ! 8c1d1850 ; rts ; nop` — r0 is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C07_C2FC).unwrap_or(0);
}
