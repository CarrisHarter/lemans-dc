//! 0x8c0b2670 — advance_4.
//!
//! `add #4,r4 ; rts ; mov r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
