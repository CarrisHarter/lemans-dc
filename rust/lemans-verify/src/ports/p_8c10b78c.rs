//! 0x8c10b78c — r0 = 0x23c; rts delay r0 = r4 + r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x23c;
    cpu.r[0] = cpu.r[4].wrapping_add(cpu.r[0]); // rts delay
}
