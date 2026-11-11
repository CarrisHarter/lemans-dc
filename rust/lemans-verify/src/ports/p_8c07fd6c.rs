//! 0x8c07fd6c — advance_12.
//!
//! `add #12,r4 ; rts ; mov r4,r0` — r4 advanced, r0 the advanced value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
