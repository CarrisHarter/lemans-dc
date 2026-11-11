//! 0x8c18e364 — r0 = 0x8c1e3cd0; rts; nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x8c1e3cd0;
}
