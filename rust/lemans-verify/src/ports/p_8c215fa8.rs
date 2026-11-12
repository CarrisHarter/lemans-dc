//! 0x8c215fa8 - bra to 0x8c215fb8. Delay slot is nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c215fb8::run(cpu, mem);
}
