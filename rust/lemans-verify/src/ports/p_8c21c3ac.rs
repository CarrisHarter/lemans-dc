//! 0x8c21c3ac - double `bra` to 0x8c21b8de. Both delay slots are nops, pr untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c21b8de::run(cpu, mem);
}
