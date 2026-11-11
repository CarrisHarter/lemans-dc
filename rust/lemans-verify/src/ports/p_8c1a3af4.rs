//! 0x8c1a3af4 — rts; delay r0 = *(r4+16).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 16).unwrap_or(0); // rts delay
}
