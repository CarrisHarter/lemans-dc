//! 0x8c215fe0 - bra to 0x8c215fea. Delay slot is nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c215fea::run(cpu, mem);
}
