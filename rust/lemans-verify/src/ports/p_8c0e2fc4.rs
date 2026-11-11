//! 0x8c0e2fc4 — load_global_indirect.
//!
//! `r0=[0x8c0e2fcc] ; rts ; r0=[r0]` — one indirection through the pool.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0E_2FCC).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
