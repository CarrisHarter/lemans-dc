//! 0x8c05abe0 — return `*(*(param + 4) + 4)`.
//!
//! Original: `mov.l @(4,r4),r0 ; rts ; mov.l @(4,r0),r0` (delay slot).
//! Side effect the C does not show: r0 is left holding the final value,
//! loaded in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // 0x8c05abe0 mov.l @(4,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0); // 0x8c05abe4 rts delay: mov.l @(4,r0),r0
}
