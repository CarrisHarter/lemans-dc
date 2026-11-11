//! 0x8c0d0ab4 — advance_28_2.
//!
//! `add #28,r4 ; rts ; mov r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
