//! 0x8c151b24 — advance_96_return_self.
//!
//! `add #96,r4 ; rts ; r0=r4` — the advance sticks.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(96);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
