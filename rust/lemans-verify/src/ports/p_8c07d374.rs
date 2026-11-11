//! 0x8c07d374 — advance_52.
//!
//! `add #52,r4 ; rts ; mov r4,r0` — r4 is advanced and r0 is the advanced value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(52);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
