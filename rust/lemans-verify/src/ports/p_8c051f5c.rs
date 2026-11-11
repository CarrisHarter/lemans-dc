//! 0x8c051f5c — `return *(u32 *)(r4 + 0x194);`
//!
//! `mov.l 0x8c051f64,r0 ; rts ; mov.l @(r0,r4),r0` — the load lives in the
//! `rts` delay slot, so r0 first becomes the pool constant 0x194 and is then
//! overwritten with the loaded word. Hidden side effect the C does not show:
//! r0 passes through 0x194 but only the final value is observable.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C051F64).unwrap(); // 0x194
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap(); // rts delay slot
}
