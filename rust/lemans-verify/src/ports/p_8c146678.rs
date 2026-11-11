//! 0x8c146678 — store_word_at_164.
//!
//! `r0=0xa4 ; rts ; mov.l r5,@(r0,r4)` — writes `r5` at `r4+0xa4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0xa4;
    let _ = mem.write_u32(cpu.r[4] + 0xa4, cpu.r[5]); // delay slot of `rts`
}
