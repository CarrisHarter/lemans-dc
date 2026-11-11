//! 0x8c237600 — r4=0x8c2c8b08; rts delay r0=*(r4+4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = 0x8c2c8b08;
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0); // rts delay
}
