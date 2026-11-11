//! 0x8c0cbe54 — copy a 76-byte struct (19 words) from r5 to r4; returns r4.
//!
//! CFG: straight-line, no branches; no FPSCR test in the real code (the
//! decompiler's SZ branches do not exist — every copy is a single fmov).
//! Stores happen in a scrambled order: floats at byte offsets 48,52,56,60,
//! 64,68,72 first (bit-exact fmov, NaN payloads preserved), then the integer
//! words at 0,4,8, 12,16,20, 36,40,44, 24,28,32.
//!
//! Hidden side effects: r0 := r4, r1 := src[8], r3 := src[6], r5 := src+24,
//! r6 := dst+24, r7 := src+36, fr0 := last float loaded (src+72).
//! r12/r13/r14 are pushed and popped (net unchanged); T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    // fmov copies first (offsets 48..=72), bit-exact
    for off in (48..=72u32).step_by(4) {
        let v = mem.read_u32(src + off).unwrap();
        let _ = mem.write_u32(dst + off, v);
    }
    // integer copies in the original's store order
    for off in [0u32, 4, 8, 12, 16, 20, 36, 40, 44, 24, 28, 32] {
        let v = mem.read_u32(src + off).unwrap();
        let _ = mem.write_u32(dst + off, v);
    }
    cpu.fr[0] = f32::from_bits(mem.read_u32(src + 72).unwrap());
    cpu.r[0] = dst; // mov r4,r0 before the pops
    cpu.r[1] = mem.read_u32(src + 32).unwrap(); // mov.l @(8,r5),r1 with r5 = src+24
    cpu.r[3] = mem.read_u32(src + 24).unwrap(); // mov.l @r5,r3 with r5 = src+24
    cpu.r[5] = src + 24;
    cpu.r[6] = dst + 24;
    cpu.r[7] = src + 36;
}
