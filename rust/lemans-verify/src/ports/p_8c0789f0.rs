//! 0x8c0789f0 — r0 := *(r4+56) (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 56).unwrap_or(0);
}
