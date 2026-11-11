//! 0x8c18ddf0 — pool_constant.
//!
//! `r0=[pool] ; rts ; nop` — returns the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c18ddf8).unwrap_or(0);
}
