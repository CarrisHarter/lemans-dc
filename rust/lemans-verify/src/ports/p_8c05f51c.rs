//! 0x8c05f51c — `add #8,r4 ; rts ; mov r4,r0`.
//!
//! Like 0x8c05f4f8 with offset 8: r4 itself is incremented and r0 is filled in
//! the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(8); // add #8,r4
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
