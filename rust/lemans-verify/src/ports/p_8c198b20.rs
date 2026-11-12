//! 0x8c198b20 — byte_at_61.
//!
//! `add #61,r4 ; rts ; r0=byte[r4]` — the advance sticks.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(61);
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // delay slot of `rts`
}
