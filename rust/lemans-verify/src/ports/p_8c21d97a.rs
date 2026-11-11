//! 0x8c21d97a — r0=r4; rts delay r0-=48.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[4];
    cpu.r[0] = cpu.r[0].wrapping_sub(48); // rts delay
}
