//! 0x8c225a04 - tail jump to memset(r4, 0, 72).
//!
//! `r6 = 72` before the jump; `r5 = 0` is the `jmp` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c225a2c).unwrap_or(0x8c010f88);
    cpu.r[6] = 72;
    cpu.r[5] = 0; // jmp delay
    crate::registry::p_8c010f88::run(cpu, mem);
}
