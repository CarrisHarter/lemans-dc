//! 0x8c18e548 — r0 = 0x8c1e3cc8; rts; nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x8c1e3cc8;
}
