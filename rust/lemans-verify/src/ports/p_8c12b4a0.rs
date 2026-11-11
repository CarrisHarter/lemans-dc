//! 0x8c12b4a0 — `return *(u32 *)(*param + 0xc);`
//!
//! Original: `mov.l @r4,r0; rts; mov.l @(12,r0),r0`. The second dereference
//! lives in the rts delay slot and always runs. No side effects beyond r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0); // rts delay slot: mov.l @(12,r0),r0
}
