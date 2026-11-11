//! 0x8c14666c — load_word_at_2004.
//!
//! `r0=0x7d4 ; rts ; r0=[r0+r4]` — a table load at `r4+0x7d4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x7d4;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0); // delay slot
}
