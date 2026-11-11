//! 0x8c056020 — return `[ [r4+0x38] + 16 ]` in r0.
//!
//! ```text
//! mov.l @(56,r4),r0 ; rts ; mov.l @(16,r0),r0   (delay slot)
//! ```
//! Side effect the C does not show: the actual load is the `rts` delay slot,
//! so r0 ends as the loaded dword, not the base pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    cpu.r[0] = mem.read_u32(base.wrapping_add(16)).unwrap_or(0); // rts delay slot: mov.l @(16,r0),r0
}
