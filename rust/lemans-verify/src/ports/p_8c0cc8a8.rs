//! 0x8c0cc8a8 — load_pool_constant_3.
//!
//! `mov.l 0x8C0C_C8B0,r0 ! 0x8c1d5928 ; rts ; nop` — `r0` is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_C8B0).unwrap_or(0);
}
