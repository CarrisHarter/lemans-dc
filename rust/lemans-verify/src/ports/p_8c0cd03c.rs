//! 0x8c0cd03c — load_pool_constant_3.
//!
//! `mov.l 0x8C0C_D044,r0 ! 0x8c1d5918 ; rts ; nop` — `r0` is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_D044).unwrap_or(0);
}
