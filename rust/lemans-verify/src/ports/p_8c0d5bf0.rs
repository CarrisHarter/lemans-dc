//! 0x8c0d5bf0 — advance_24_2.
//!
//! `add #24,r4 ; rts ; mov r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(24);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
