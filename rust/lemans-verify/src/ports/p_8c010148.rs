//! 0x8c010148 — `return *DAT_8c010150 + 1;`
//!
//! `mov.l 0x8c010150,r0 ; mov.l @r0,r0 ; rts ; add #1,r0`
//! The C hides two things: the double indirection through the literal pool,
//! and the `add #1,r0` sitting in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let ptr = mem.read_u32(0x8C010150).unwrap();
    cpu.r[0] = mem.read_u32(ptr).unwrap();
    cpu.r[0] = cpu.r[0].wrapping_add(1); // delay slot of `rts`
}
