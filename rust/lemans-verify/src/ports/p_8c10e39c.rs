//! 0x8c10e39c — load_word_via_field_48.
//!
//! `r0=[r4] ; rts ; r0=[r0+48]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 48).unwrap_or(0); // delay slot of `rts`
}
