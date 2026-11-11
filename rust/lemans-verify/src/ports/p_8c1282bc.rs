//! 0x8c1282bc — double read at `param + 0x40` (Ghidra's fr1 is really fr0).
//!
//! Original: `add #64,r4; rts; fmov @r4,fr0`. Hidden side effects: r4 += 64
//! sticks, fr0 := raw bits at the shifted pointer (delay slot, always runs).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(64); // add #64,r4
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0)); // rts delay slot: fmov @r4,fr0
}
