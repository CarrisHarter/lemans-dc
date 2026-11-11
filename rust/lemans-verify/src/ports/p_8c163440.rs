//! 0x8c163440 — r4 += 88; rts delay r0 = *r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(88);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // rts delay
}
