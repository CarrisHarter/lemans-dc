//! 0x8c056004 — load fr0 from `[ [r4+0x38] + 12 ]`.
//!
//! ```text
//! mov.l @(56,r4),r0 ; add #12,r0 ; rts ; fmov @r0,fr0   (delay slot)
//! ```
//! Side effect the C does not show: r0 is left at base+12 and the load is the
//! `rts` delay slot, so fr0 is the only architectural result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    cpu.r[0] = base.wrapping_add(12); // mov.l @(56,r4),r0 ; add #12,r0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // rts delay slot: fmov @r0,fr0
}
