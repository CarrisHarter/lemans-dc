//! 0x8c0d474c — advance_61_load_byte.
//!
//! `add #61,r4 ; rts ; mov.b @r4,r0` — `r4` is advanced before the delay-slot
//! byte load, which sign-extends.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(61);
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // delay slot
}
