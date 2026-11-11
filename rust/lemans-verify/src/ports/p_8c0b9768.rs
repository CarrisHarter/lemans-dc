//! 0x8c0b9768 — zero_word_and_byte.
//!
//! `r1=0 ; [r4+4]=r1 ; r0=r4 ; rts ; [r4]=r1` — clears the byte at `[r4]` and
//! the word at `+4`, returning `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = 0;
    let _ = mem.write_u32(base + 4, 0);
    cpu.r[0] = base;
    let _ = mem.write_u8(base, 0); // delay slot of `rts`
}
