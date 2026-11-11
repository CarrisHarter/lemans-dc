//! 0x8c104768 — advance_32_load_byte.
//!
//! `add #32,r4 ; mov.b @r4,r0 ; rts ; extu.b r0,r0` — a zero-extended byte at
//! the advanced pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(32);
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
    cpu.r[0] &= 0xFF; // delay slot: extu.b r0,r0
}
