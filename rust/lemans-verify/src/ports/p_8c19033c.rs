//! 0x8c19033c — deref_pool_pointer.
//!
//! `r0=[0x8c190344] ; rts ; r0=[r0]` — one dereference of the pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c190344).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
