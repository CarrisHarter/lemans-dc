//! 0x8c0c1730 — load_byte_28_via_36.
//!
//! `r0=[r4+36] ; r0+=28 ; r0=[r0] ; rts ; extu.b r0,r0` — a zero-extended byte
//! reached through the +36 pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] &= 0xFF; // delay slot: extu.b r0,r0
}
