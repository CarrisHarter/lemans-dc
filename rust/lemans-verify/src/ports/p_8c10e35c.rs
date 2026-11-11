//! 0x8c10e35c — load_word_via_field_68.
//!
//! `r0=[r4] ; r0+=68 ; rts ; r0=[r0]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(68);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
