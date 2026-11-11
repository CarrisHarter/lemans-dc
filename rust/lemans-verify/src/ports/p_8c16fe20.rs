//! 0x8c16fe20 — store_fr4_at_12_and_8.
//!
//! `[r4+12]=fr4 ; add #8,r4 ; rts ; [r4]=fr4` — two float stores, the second
//! at the advanced pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]);
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of `rts`
}
