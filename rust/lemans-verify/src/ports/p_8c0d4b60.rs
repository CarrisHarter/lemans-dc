//! 0x8c0d4b60 — zero_field_52.
//!
//! `mov #0,r0 ; rts ; mov.l r0,@(52,r4)` — clears the word at `r4+52`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4] + 52, 0); // delay slot of `rts`
}
