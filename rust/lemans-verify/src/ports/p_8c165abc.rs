//! 0x8c165abc — zero_field.
//!
//! `mov #0,r0 ; rts ; [r4]=r0` — zeroes the word at `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // delay slot of `rts`
}
