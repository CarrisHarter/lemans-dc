//! 0x8c055fec — store fr4 at `[ [r4+0x38] + 8 ]`.
//!
//! ```text
//! mov.l @(56,r4),r0 ; add #8,r0 ; rts ; fmov fr4,@r0   (delay slot)
//! ```
//! Side effect the C does not show: r0 is left holding base+8, the interior
//! pointer, and the store itself is the `rts` delay slot. The FPSCR.SZ branch
//! in the decompile is spurious — the oracle's `fmov.s` is a fixed 32-bit move.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    cpu.r[0] = base.wrapping_add(8); // mov.l @(56,r4),r0 ; add #8,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits()); // rts delay slot: fmov fr4,@r0
}
