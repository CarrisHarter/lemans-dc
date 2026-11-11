//! 0x8c05f510 — `mov #16,r0 ; mov #1,r1 ; rts ; mov.b r1,@(r0,r4)`.
//!
//! Same shape as 0x8c05f508 with the constant 1; the store is the rts delay
//! slot and r0 = 16 / r1 = 1 are left behind.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 16; // mov #16,r0
    cpu.r[1] = 1; // mov #1,r1
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // rts delay slot
}
