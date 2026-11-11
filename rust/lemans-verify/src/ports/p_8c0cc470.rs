//! 0x8c0cc470 — return_self_2.
//!
//! `rts ; mov r4,r0` — returns the argument.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
