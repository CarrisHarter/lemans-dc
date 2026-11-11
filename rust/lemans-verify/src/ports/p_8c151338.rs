//! 0x8c151338 — r4 += 32; rts delay r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(32);
    cpu.r[0] = cpu.r[4]; // rts delay
}
