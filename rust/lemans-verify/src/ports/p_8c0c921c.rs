//! 0x8c0c921c — load_double_indirect.
//!
//! `r0=[r4] ; rts ; r0=[r0]` — two levels of indirection.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
