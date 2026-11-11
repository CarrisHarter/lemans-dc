//! 0x8c01e484 — `return param;`
//!
//! `rts ; mov r4,r0` — the return value is copied in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
