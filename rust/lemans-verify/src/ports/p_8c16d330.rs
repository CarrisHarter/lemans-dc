//! 0x8c16d330 — copy_word_return_self.
//!
//! `r1=[r5] ; r0=r4 ; rts ; [r4]=r1` — copies one word from `r5` to `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // delay slot of `rts`
}
