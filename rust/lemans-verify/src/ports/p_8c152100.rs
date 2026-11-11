//! 0x8c152100 — advance_60_return_self.
//!
//! `add #60,r4 ; rts ; r0=r4` — the advance sticks.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(60);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
