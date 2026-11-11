//! 0x8c05f4f8 — `add #24,r4 ; rts ; mov r4,r0`.
//!
//! The C shows only `return param_1 + 0x18`; the original also leaves the
//! argument register r4 itself incremented (r0 is filled in the rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
