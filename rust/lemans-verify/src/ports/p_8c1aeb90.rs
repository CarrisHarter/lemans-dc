//! 0x8c1aeb90 - bra to 0x8c1aeb9e. Delay slot is nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c1aeb9e::run(cpu, mem);
}
