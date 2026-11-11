//! 0x8c10e344 — load_byte_via_field_82.
//!
//! `r0=[r4] ; r0+=82 ; r0=[r0] ; rts ; extu.b r0,r0` — a zero-extended
//! byte through the pointer at `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(82);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] &= 0xFF; // delay slot: extu.b r0,r0
}
