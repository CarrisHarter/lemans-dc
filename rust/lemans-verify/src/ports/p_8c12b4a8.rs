//! 0x8c12b4a8 — `return *(u32 *)(*param + 8);`
//!
//! Original: `mov.l @r4,r0; rts; mov.l @(8,r0),r0`. Second dereference in the
//! rts delay slot, always runs. No side effects beyond r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = mem.read_u32(cpu.r[0] + 8).unwrap_or(0); // rts delay slot: mov.l @(8,r0),r0
}
