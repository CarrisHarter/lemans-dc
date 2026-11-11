//! 0x8c060d84 — copy 16 bytes (4 fmov.s) from [[r4]] to [[r15]].
//!
//! The decompiled FPSCR.SZ branch is spurious: the straight-line code always
//! does single-precision fmovs. Dest comes from the stack ([r15]), src from
//! [r4]. Leftovers the C does not show: r6 = dst+12, r5 = src+12, r4 = dst+4,
//! r3 = src+4, r1 = dst+8, r0 = src+8, and fr0 holds the raw bits of
//! mem32[src+12] (the last load lands in the rts delay slot's fmov).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r6 (dst)
    cpu.r[5] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r5 (src)
    cpu.r[1] = cpu.r[6].wrapping_add(8); // mov r6,r1 ; add #8,r1
    let w0 = mem.read_u32(cpu.r[5]).unwrap_or(0); // fmov @r5,fr0
    cpu.fr[0] = f32::from_bits(w0);
    cpu.r[3] = cpu.r[5]; // mov r5,r3
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    let _ = mem.write_u32(cpu.r[6], w0); // fmov fr0,@r6
    cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
    let w1 = mem.read_u32(cpu.r[3]).unwrap_or(0); // fmov @r3,fr0
    cpu.fr[0] = f32::from_bits(w1);
    cpu.r[4] = cpu.r[6].wrapping_add(4); // mov r6,r4 ; add #4,r4
    let _ = mem.write_u32(cpu.r[4], w1); // fmov fr0,@r4
    cpu.r[0] = cpu.r[0].wrapping_add(8); // add #8,r0
    let w2 = mem.read_u32(cpu.r[0]).unwrap_or(0); // fmov @r0,fr0
    cpu.fr[0] = f32::from_bits(w2);
    cpu.r[5] = cpu.r[5].wrapping_add(12); // add #12,r5
    cpu.r[6] = cpu.r[6].wrapping_add(12); // add #12,r6
    let _ = mem.write_u32(cpu.r[1], w2); // fmov fr0,@r1
    let w3 = mem.read_u32(cpu.r[5]).unwrap_or(0); // fmov @r5,fr0
    cpu.fr[0] = f32::from_bits(w3);
    let _ = mem.write_u32(cpu.r[6], w3); // rts delay slot: fmov fr0,@r6
}
