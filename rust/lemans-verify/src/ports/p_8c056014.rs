//! 0x8c056014 — store fr4 at `[r4+0x38]` and fr5 at `[r4+0x38]+4`.
//!
//! ```text
//! mov.l @(56,r4),r0 ; fmov fr4,@r0
//! mov.l @(56,r4),r0 ; add #4,r0 ; rts ; fmov fr5,@r0   (delay slot)
//! ```
//! Side effects the C does not show: [r4+0x38] is re-loaded for the second
//! store, r0 is left at base+4, and the second store is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    let _ = mem.write_u32(base, cpu.fr[4].to_bits()); // fmov fr4,@r0
    cpu.r[0] = base.wrapping_add(4); // mov.l @(56,r4),r0 ; add #4,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[5].to_bits()); // rts delay slot: fmov fr5,@r0
}
