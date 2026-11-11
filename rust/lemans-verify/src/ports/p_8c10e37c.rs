//! 0x8c10e37c — load_word_via_field_60.
//!
//! `r0=[r4] ; rts ; r0=[r0+60]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 60).unwrap_or(0); // delay slot of `rts`
}
