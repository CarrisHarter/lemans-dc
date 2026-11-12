//! 0x8c2199e6 - tail jump to 0x8c21b034. pr is not touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c219a1c).unwrap_or(0x8c21b034); // mov.l 0x8c219a1c,r3
    crate::registry::p_8c21b034::run(cpu, mem); // jmp delay is nop
}
