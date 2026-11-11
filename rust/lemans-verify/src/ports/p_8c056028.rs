//! 0x8c056028 — store r5 at `[ [r4+0x38] + 16 ]`.
//!
//! ```text
//! mov.l @(56,r4),r0 ; rts ; mov.l r5,@(16,r0)   (delay slot)
//! ```
//! Side effect the C does not show: r0 is left holding the *base* pointer
//! (the delay slot stores at r0+16), not base+16.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    cpu.r[0] = base; // mov.l @(56,r4),r0
    let _ = mem.write_u32(base.wrapping_add(16), cpu.r[5]); // rts delay slot: mov.l r5,@(16,r0)
}
