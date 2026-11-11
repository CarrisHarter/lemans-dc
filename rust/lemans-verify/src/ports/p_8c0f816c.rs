//! 0x8c0f816c — advance_56.
//!
//! `add #56,r4 ; rts ; mov r4,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(56);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
