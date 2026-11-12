//! 0x8c219c92 - tail jump to 0x8c21b04e. pr is not touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c219d90).unwrap_or(0x8c21b04e); // mov.l 0x8c219d90,r3
    crate::registry::p_8c21b04e::run(cpu, mem); // jmp delay is nop
}
