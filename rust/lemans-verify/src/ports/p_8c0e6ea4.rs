//! 0x8c0e6ea4 — load_global_field_12.
//!
//! `r0=[0x8c0e6eac] ; rts ; r0=[r0+12]` — a field of the pool target.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0E_6EAC).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0); // delay slot of `rts`
}
