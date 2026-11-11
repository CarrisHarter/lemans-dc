//! 0x8c23a544 — rts; delay r0=*(r4+44).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 44).unwrap_or(0); // rts delay
}
