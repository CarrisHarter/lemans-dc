//! 0x8c1514d8 — r4 += 4; rts delay r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.r[0] = cpu.r[4]; // rts delay
}
