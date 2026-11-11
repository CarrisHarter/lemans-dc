//! 0x8c0c0d58 — load_global_byte.
//!
//! `r0=[0x8c0c0d60] ; r0=[r0] ; rts ; extu.b r0,r0` — the zero-extended byte at
//! the pool target.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_0D60).unwrap_or(0);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] &= 0xFF; // delay slot: extu.b r0,r0
}
