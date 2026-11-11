//! 0x8c18ca20 — advance_36_return_self.
//!
//! `add #36,r4 ; rts ; r0=r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(36);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
