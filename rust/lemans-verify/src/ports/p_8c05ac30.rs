//! 0x8c05ac30 — store r5 at `*(r4+4) + 0x14`.
//!
//! Original: `mov.l @(4,r4),r0 ; rts ; mov.l r5,@(20,r0)` (delay slot).
//! Side effect the C does not show: r0 is left holding the loaded base
//! pointer, and the store happens in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // 0x8c05ac30 mov.l @(4,r4),r0
    let _ = mem.write_u32(cpu.r[0].wrapping_add(0x14), cpu.r[5]); // 0x8c05ac34 rts delay: mov.l r5,@(20,r0)
}
