//! 0x8c163464 — load_byte_indexed_global.
//!
//! `r0=[0x8c16346c] ; r0=byte[r0+r4] ; rts ; extu.b r0,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c16346c).unwrap_or(0);
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff; // delay slot of `rts`
}
