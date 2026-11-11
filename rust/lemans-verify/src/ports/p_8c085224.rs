//! 0x8c085224 — zero_two_floats_then_word.
//!
//! `[r4+4]=0.0 ; [r4+8]=0.0 ; r0=0 ; rts ; [r4]=0` — the word store is the
//! delay slot, so the write order is the reverse of 0x8c0851e0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = base.wrapping_add(4);
    cpu.fr[0] = 0.0;
    cpu.r[0] = base;
    let _ = mem.write_f32(cpu.r[1], 0.0);
    cpu.r[0] = base.wrapping_add(8);
    let _ = mem.write_f32(cpu.r[0], 0.0);
    cpu.r[0] = 0;
    let _ = mem.write_u32(base, 0); // delay slot of `rts`
}
