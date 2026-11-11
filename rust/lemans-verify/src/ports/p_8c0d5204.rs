//! 0x8c0d5204 — zero_three_words_2.
//!
//! `r1=0 ; [r4+4]=r1 ; r0=r4 ; [r4+8]=r1 ; rts ; [r4]=r1` — the first store is
//! the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = 0;
    let _ = mem.write_u32(base + 4, 0);
    cpu.r[0] = base;
    let _ = mem.write_u32(base + 8, 0);
    let _ = mem.write_u32(base, 0); // delay slot of `rts`
}
