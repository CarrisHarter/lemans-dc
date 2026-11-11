//! 0x8c0ccef0 — load_pool_constant_3.
//!
//! `mov.l 0x8C0C_CEF8,r0 ! 0x8c1d5920 ; rts ; nop` — `r0` is the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_CEF8).unwrap_or(0);
}
