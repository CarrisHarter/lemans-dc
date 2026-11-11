//! 0x8c155e14 — global_pointer_deref.
//!
//! `r0=[0x8c155e1c] ; rts ; r0=[r0]` — one level through the pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c155e1c).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
