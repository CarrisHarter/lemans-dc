//! 0x8c220f96 - tail jump to 0x8c224be2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c221050).unwrap_or(0x8c224be2);
    crate::registry::p_8c224be2::run(cpu, mem); // jmp delay is nop
}
