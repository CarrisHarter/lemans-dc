//! 0x8c0d0aac — advance_40_2.
//!
//! `add #40,r4 ; rts ; mov r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(40);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
