//! 0x8c0b2ca0 — cross product: load 3 floats from `r4` and 3 from `r5`,
//! subtract pairwise, scale by `fr4` and accumulate with `fmac`, then store
//! the 3 results to the struct at `[r15]` (a stack argument here).
//!
//! Original:
//! ```text
//! fmov @r4+,fr1 ; fmov @r5+,fr5 ; fmov @r4+,fr2 ; fmov @r5+,fr6
//! fsub fr1,fr5 ; fmov @r4+,fr3 ; fmov @r5+,fr7 ; fsub fr2,fr6
//! mov.l @r15,r0 ; fmov fr4,fr0 ; fsub fr3,fr7
//! fmac fr0,fr5,fr1 ; pref @r0 ; fmac fr0,fr6,fr2 ; fmac fr0,fr7,fr3
//! fmov fr1,@r0 ; add #4,r0 ; fmov fr2,@r0 ; add #4,r0 ; rts ; fmov fr3,@r0
//! ```
//! Side effects the C hides: `r4`/`r5` are post-incremented (end +12),
//! `r0` ends at out+8, `fr0` keeps `fr4`, and `fr5/fr6/fr7` keep the pairwise
//! differences. `pref` has no architectural effect; T is never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[5] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[6] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[5] = cpu.fr[5] - cpu.fr[1]; // fsub fr1,fr5
    cpu.fr[3] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[7] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[6] = cpu.fr[6] - cpu.fr[2]; // fsub fr2,fr6
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // out pointer (stack arg)
    cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0
    cpu.fr[7] = cpu.fr[7] - cpu.fr[3]; // fsub fr3,fr7
    cpu.fr[1] = cpu.fr[0].mul_add(cpu.fr[5], cpu.fr[1]); // fmac fr0,fr5,fr1
    // pref @r0: cache hint, no architectural effect
    cpu.fr[2] = cpu.fr[0].mul_add(cpu.fr[6], cpu.fr[2]); // fmac fr0,fr6,fr2
    cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[7], cpu.fr[3]); // fmac fr0,fr7,fr3
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[2]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[3]); // rts delay slot
}
