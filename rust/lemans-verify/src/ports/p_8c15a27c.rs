//! 0x8c15a27c — double_indirect_global.
//!
//! `r0=[0x8c15a284] ; r0=[r0] ; rts ; r0=[r0]` — two dereferences from the
//! pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c15a284).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // delay slot of `rts`
}
