//! 0x8c0203f8 — `*(u32 *)(0x10c + param_1) = param_2;`
//!
//! `mov.l 0x8c020400,r0 ; rts ; mov.l r5,@(r0,r4)` — the base 0x10c comes
//! from the literal pool into r0 (where it stays as a leftover), and the
//! store is the delay slot of `rts`, so it always executes.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(0x8c020400).unwrap_or(0);
    cpu.r[0] = base;
    let _ = mem.write_u32(base.wrapping_add(cpu.r[4]), cpu.r[5]);
}
