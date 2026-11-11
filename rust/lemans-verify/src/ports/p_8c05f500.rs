//! 0x8c05f500 — `add #16,r4 ; mov.b @r4,r0 ; rts ; extu.b r0,r0`.
//!
//! The C shows only the return byte; r4 is also left incremented by 16, and the
//! return value is produced by the rts delay slot `extu.b r0,r0` (the load
//! itself is a mid-function instruction, not the delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(16); // add #16,r4
    let b = mem.read_u8(cpu.r[4]).unwrap_or(0); // mov.b @r4,r0
    cpu.r[0] = b as u32; // rts delay slot: extu.b r0,r0
}
