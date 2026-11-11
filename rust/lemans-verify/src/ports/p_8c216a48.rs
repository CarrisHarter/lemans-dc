//! 0x8c216a48 — r0 = 0x8c23c010; bare rts;nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0x8c23c010;
}
