//! 0x8c18fbc8 — zero_word_return_self.
//!
//! `mov #0,r0 ; [r4]=r0 ; rts ; r0=r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
