//! 0x8c15d358 — r0 = 0x8c1dca68; rts; nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x8c1dca68;
}
