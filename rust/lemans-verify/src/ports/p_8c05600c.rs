//! 0x8c05600c — store fr4 at `[ [r4+0x38] + 12 ]`.
//!
//! ```text
//! mov.l @(56,r4),r0 ; add #12,r0 ; rts ; fmov fr4,@r0   (delay slot)
//! ```
//! Side effect the C does not show: r0 is left at base+12 and the store is the
//! `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    cpu.r[0] = base.wrapping_add(12); // mov.l @(56,r4),r0 ; add #12,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits()); // rts delay slot: fmov fr4,@r0
}
