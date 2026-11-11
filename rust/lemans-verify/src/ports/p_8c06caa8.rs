//! 0x8c06caa8 — advance the argument by 12 and return it.
//!
//! `add #12,r4 ; rts ; mov r4,r0`
//! `r4` itself is advanced and `r0` is the advanced value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
