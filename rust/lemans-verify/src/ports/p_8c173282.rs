//! 0x8c173282 — load_global_pointer.
//!
//! `r3=[0x8c173290] ; rts ; r0=[r3]` — one dereference of the pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c173290).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // delay slot of `rts`
}
