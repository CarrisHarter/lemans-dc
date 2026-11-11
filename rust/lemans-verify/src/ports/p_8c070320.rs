//! 0x8c070320 — offset_0xb0.
//!
//! `mov.l 0x8c070328,r0 ! 0xb0 ; rts ; add r4,r0` — the add is the delay slot, so r0 = 0xb0 + r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0xb0;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
