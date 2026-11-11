//! 0x8c04b358 — `return **(undefined4 **)(*(int *)(param_1 + 0x20) + 0x3c);`
//!
//! ```text
//! mov.l @(32,r4),r0
//! mov.l @(60,r0),r0
//! rts
//! mov.l @r0,r0       ; delay slot — the second dereference
//! ```
//! Hidden side effect: the rts delay slot performs the final `*ptr` load, so
//! the C's `**` happens *after* the return instruction, not before it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = mem.read_u32(cpu.r[4].wrapping_add(0x20)).unwrap();
    let q = mem.read_u32(p.wrapping_add(0x3c)).unwrap();
    cpu.r[0] = mem.read_u32(q).unwrap(); // delay slot of `rts`
}
