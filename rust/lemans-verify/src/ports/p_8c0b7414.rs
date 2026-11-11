//! 0x8c0b7414 — delta_12_minus_8.
//!
//! `r0=[r4+12] ; r1=[r4+8] ; rts ; r0-=r1` — the subtraction is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // delay slot of `rts`
}
