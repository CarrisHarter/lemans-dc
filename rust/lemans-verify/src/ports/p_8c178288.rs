//! 0x8c178288 — rts; delay r0 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0; // rts delay
}
