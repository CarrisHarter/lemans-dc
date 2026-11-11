//! 0x8c1282a4 — `fr0 = *(f32*)(r4+0x4c)`; the load is in the delay slot.
//!
//! ```text
//! add  #76,r4
//! rts
//! fmov @r4,fr0    ; delay slot: fr0 = 32-bit load at r4+0x4c
//! ```
//! Hidden side effects: r4 += 0x4c (not shown in the C) and fr0 is written by
//! the rts delay slot, bit-exact (NaN payloads survive).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(0x4C); // add #76,r4
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap(); // delay slot: fmov @r4,fr0
}
