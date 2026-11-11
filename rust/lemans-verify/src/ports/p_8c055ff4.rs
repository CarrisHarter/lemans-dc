//! 0x8c055ff4 — copy the float pair at `[r4+0x38]` to `[r5]` and `[r6]`.
//!
//! ```text
//! mov.l @(56,r4),r0 ; fmov @r0,fr0 ; fmov fr0,@r5
//! mov.l @(56,r4),r0 ; add #4,r0 ; fmov @r0,fr0 ; rts ; fmov fr0,@r6 (delay)
//! ```
//! Side effects the C does not show: r0 is re-loaded from [r4+0x38] a second
//! time and left at base+4, fr0 ends as the *second* float, and the store to
//! [r6] is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap_or(0);
    let v0 = mem.read_u32(base).unwrap_or(0);
    cpu.fr[0] = f32::from_bits(v0); // fmov @r0,fr0
    let _ = mem.write_u32(cpu.r[5], v0); // fmov fr0,@r5
    cpu.r[0] = base.wrapping_add(4); // mov.l @(56,r4),r0 ; add #4,r0
    let v1 = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.fr[0] = f32::from_bits(v1); // fmov @r0,fr0
    let _ = mem.write_u32(cpu.r[6], v1); // rts delay slot: fmov fr0,@r6
}
