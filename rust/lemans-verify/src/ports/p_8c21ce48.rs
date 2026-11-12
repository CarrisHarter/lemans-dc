//! 0x8c21ce48 - tail jump to 0x8c21cf1c.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21cf0c).unwrap_or(0x8c21cf1c); // mov.l 0x8c21cf0c,r3
    crate::registry::p_8c21cf1c::run(cpu, mem); // jmp @r3; delay nop
}
