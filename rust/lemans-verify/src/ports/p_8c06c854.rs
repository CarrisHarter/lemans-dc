//! 0x8c06c854 — store the argument into a global slot.
//!
//! `mov.l 0x8c06c85c,r0 ; rts ; mov.l r4,@r0`
//! The store is the `rts` delay slot and `r0` is left holding the pool
//! address 0x8c23f438, not the argument.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C06_C85C).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
