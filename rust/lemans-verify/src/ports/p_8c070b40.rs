//! 0x8c070b40 — store the argument into a global slot.
//!
//! `mov.l 0x8c070b48,r0 ! 8c240458 ; rts ; mov.l r4,@r0`
//! The store is the `rts` delay slot; `r0` keeps the pool address.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C07_0B48).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
