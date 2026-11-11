//! 0x8c0f02fc — store_word_at_64.
//!
//! `mov #64,r0 ; rts ; mov.l r5,@(r0,r4)` — writes `r5` at `r4+64`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 64;
    let _ = mem.write_u32(cpu.r[4] + 64, cpu.r[5]); // delay slot of `rts`
}
