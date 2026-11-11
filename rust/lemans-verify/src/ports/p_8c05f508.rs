//! 0x8c05f508 — `mov #16,r0 ; mov #0,r1 ; rts ; mov.b r1,@(r0,r4)`.
//!
//! The store is the rts delay slot; the C shows only the store. Leftover
//! registers r0 = 16 and r1 = 0 are the addressing values the C never mentions.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 16; // mov #16,r0
    cpu.r[1] = 0; // mov #0,r1
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // rts delay slot
}
