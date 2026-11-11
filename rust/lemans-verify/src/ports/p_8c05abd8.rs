//! 0x8c05abd8 — return `**(param + 4)`.
//!
//! Original: `mov.l @(4,r4),r0 ; rts ; mov.l @r0,r0` (delay slot).
//! Side effect the C does not show: r0 is left holding the final value,
//! loaded in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // 0x8c05abd8 mov.l @(4,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // 0x8c05abdc rts delay: mov.l @r0,r0
}
