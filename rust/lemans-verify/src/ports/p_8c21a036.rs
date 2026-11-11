//! 0x8c21a036 — global_word.
//!
//! `r3=[0x8c21a0b4] ; rts ; r0=[r3]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21a0b4).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // delay slot of `rts`
}
