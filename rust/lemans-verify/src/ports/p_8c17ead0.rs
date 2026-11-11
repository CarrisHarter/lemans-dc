//! 0x8c17ead0 — store_r5_at_72.
//!
//! `mov #72,r0 ; rts ; [r0+r4]=r5` — a single indexed store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 72;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // delay slot of `rts`
}
