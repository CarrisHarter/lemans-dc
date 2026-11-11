//! 0x8c16cbc4 — advance_8_return_self.
//!
//! `add #8,r4 ; rts ; r0=r4` — the advance sticks.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
