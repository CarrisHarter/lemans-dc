//! 0x8c15b410 — r0 = 0x8c1dcc58; rts; nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x8c1dcc58;
}
