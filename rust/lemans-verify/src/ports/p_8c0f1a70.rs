//! 0x8c0f1a70 — load_pool_constant_6.
//!
//! `mov.l 0x8c0f1a78,r0 ! 8c1d76a8 ; rts ; nop` — `r0` is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0F_1A78).unwrap_or(0);
}
