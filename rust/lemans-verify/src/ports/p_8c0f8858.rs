//! 0x8c0f8858 — advance_20_3.
//!
//! `add #20,r4 ; rts ; mov r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
