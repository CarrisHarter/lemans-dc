//! 0x8c060e40 — copy 8 bytes (2 fmov.s) from [r4] to [r5] and [r6].
//!
//! The decompiled FPSCR.SZ branch is spurious; the code always does
//! single-precision fmovs. Leftovers the C does not show: r4 ends at
//! r4+4 (the add runs between the two loads), fr0 holds the raw bits of
//! mem32[r4+4], and the second store ([r6]) is the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let w0 = mem.read_u32(cpu.r[4]).unwrap_or(0); // fmov @r4,fr0
    cpu.fr[0] = f32::from_bits(w0);
    let _ = mem.write_u32(cpu.r[5], w0); // fmov fr0,@r5
    cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
    let w1 = mem.read_u32(cpu.r[4]).unwrap_or(0); // fmov @r4,fr0
    cpu.fr[0] = f32::from_bits(w1);
    let _ = mem.write_u32(cpu.r[6], w1); // rts delay slot: fmov fr0,@r6
}
