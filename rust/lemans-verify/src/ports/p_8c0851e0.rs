//! 0x8c0851e0 — zero_word_and_two_floats.
//!
//! `[r4]=0 ; [r4+4]=0.0 ; [r4+8]=0.0 ; rts ; r0=r4` — the word store comes
//! first, the two float stores after, and `r0` ends as `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = 0;
    cpu.r[1] = base;
    let _ = mem.write_u32(base, 0);
    cpu.r[1] = base.wrapping_add(4);
    cpu.fr[0] = 0.0;
    cpu.r[0] = base;
    let _ = mem.write_f32(cpu.r[1], 0.0);
    cpu.r[0] = base.wrapping_add(8);
    let _ = mem.write_f32(cpu.r[0], 0.0);
    cpu.r[0] = base; // delay slot of `rts`
}
