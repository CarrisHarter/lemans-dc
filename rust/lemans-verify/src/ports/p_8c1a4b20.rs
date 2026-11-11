//! 0x8c1a4b20 — r0 = 0x8c2111d8; rts delay r0 = *r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8c2111d8;
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // rts delay
}
