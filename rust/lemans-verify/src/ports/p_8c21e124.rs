//! 0x8c21e124 — r0=r5; rts delay r0-=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[4]); // rts delay
}
