//! 0x8c10610c — r0 = *r4; rts delay r0 = *r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // rts delay
}
