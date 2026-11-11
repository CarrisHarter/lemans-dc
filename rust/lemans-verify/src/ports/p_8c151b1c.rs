//! 0x8c151b1c — advance_108_return_self.
//!
//! `add #108,r4 ; rts ; r0=r4` — the advance sticks.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(108);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
