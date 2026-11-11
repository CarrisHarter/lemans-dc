//! 0x8c15e584 — r0 = 0x8c1dcd58; rts; nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x8c1dcd58;
}
