//! 0x8c0274d8 — `return **(u32 **)(DAT_8c0274e0 + param_1);`
//!
//! `mov.l 0x8c0274e0,r0 ; mov.l @(r0,r4),r0 ; rts ; mov.l @r0,r0`
//! The C shows a double dereference but not that the final `mov.l @r0,r0`
//! is the `rts` delay slot: r0 ends as the *second* load, and the first
//! load's pointer is destroyed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0274E0).unwrap(); // literal pool: 0x174
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // delay slot of `rts`
}
