//! 0x8c0654b4 — `return r4 + 0x40`.
//!
//! Original: `add #64,r4 ; rts ; mov r4,r0`
//! The add mutates the argument register itself (wrapping); the delay slot
//! copies the *incremented* r4 into r0 as the return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = (cpu.r[4] as i32).wrapping_add(0x40) as u32; // add #64,r4
    cpu.r[0] = cpu.r[4]; // delay slot of rts
}
