//! 0x8c216dbc - bra to 0x8c216dc8. Delay slot is nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c216dc8::run(cpu, mem);
}
