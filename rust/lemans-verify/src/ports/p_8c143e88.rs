//! 0x8c143e88 — store_word_at_92.
//!
//! `mov #92,r0 ; rts ; mov.l r5,@(r0,r4)` — writes `r5` at `r4+92`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 92;
    let _ = mem.write_u32(cpu.r[4] + 92, cpu.r[5]); // delay slot of `rts`
}
