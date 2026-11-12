//! 0x8c221eee - tail jump to 0x8c21d6bc.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c221f14).unwrap_or(0x8c21d6bc); // mov.l 0x8c221f14,r2
    crate::registry::p_8c21d6bc::run(cpu, mem); // jmp @r2; delay nop
}
