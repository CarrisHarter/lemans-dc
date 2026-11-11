//! 0x8c163450 — byte_at_92.
//!
//! `add #92,r4 ; r0=byte[r4] ; rts ; extu.b r0,r0` — the advance sticks.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff; // delay slot of `rts`
}
