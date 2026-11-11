//! 0x8c152084 — store_fr4_at_72.
//!
//! `add #72,r4 ; rts ; [r4]=fr4` — the store lands at the advanced pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(72);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of `rts`
}
