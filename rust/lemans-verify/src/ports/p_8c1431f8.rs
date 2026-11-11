//! 0x8c1431f8 — store_word_at_132.
//!
//! `r0=0x84 ; rts ; mov.l r5,@(r0,r4)` — writes `r5` at `r4+0x84`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x84;
    let _ = mem.write_u32(cpu.r[4] + 0x84, cpu.r[5]); // delay slot of `rts`
}
