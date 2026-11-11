//! 0x8c0b90e8 — load_byte_zext.
//!
//! `r0=[r4] ; rts ; extu.b r0,r0` — the zero-extended byte at `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
    cpu.r[0] &= 0xFF; // delay slot: extu.b r0,r0
}
