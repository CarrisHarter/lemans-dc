//! 0x8c22c3a0 - tail jump to 0x8c236e60. Delay slot is nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c22c3c0).unwrap_or(0x8c236e60);
    crate::registry::p_8c236e60::run(cpu, mem);
}
