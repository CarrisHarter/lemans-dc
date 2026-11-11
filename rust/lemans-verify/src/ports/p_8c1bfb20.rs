//! 0x8c1bfb20 — rts; delay r0 = extu.b(byte at r4+0xe0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4] + 0xe0).unwrap_or(0) as u32; // rts delay
}
