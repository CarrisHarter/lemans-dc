//! 0x8c1745b8 — global_pointer.
//!
//! `r0=[0x8c1745f4] ; rts ; nop` — returns the address `0x8c2b552c`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1745f4).unwrap_or(0);
}
