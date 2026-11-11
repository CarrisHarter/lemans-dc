//! 0x8c0da8c0 — initialise a 28-word (112-byte) transform at r4.
//!
//! Straight-line. r6 = @0x8c0da9bc (a pointer global = 0x8c1d0f28); four
//! constants are read from 0x8c0da9c0..0x8c0da9cc via `mova`. Layout written
//! to r4+0..r4+108:
//!
//! ```text
//! fldi1 fr3 ; fmov fr3,@r4          [r4+  0] = 1.0
//! [r4+ 4..16] = g[0..3]              (g = @0x8c0da9bc)
//! [r4+20] = c0  [r4+24] = c1  [r4+28] = 0.0 (fldi0 fr1)
//! [r4+32] = c2  [r4+36] = c3  [r4+40] = 1.0 (fr3)  [r4+44] = 0
//! [r4+48] = fr2 (= c0)
//! [r4+52..64] = g[0..3]  [r4+68..80] = g[0..3]  [r4+84..96] = g[0..3]
//! [r4+100] = [r4+104] = [r4+108] = 0.0 (fr1)
//! ```
//!
//! Hidden side effects the decompiled C does not show: fr1 = 0.0 and
//! fr3 = 1.0 (fldi clobbers the seeded values), fr2 = c0, fr0 = g[3] (last
//! load), r0 = r4 (return), r1 = r4+104, r3 = g+8, r5 = g+4, r7 = r4+92,
//! r2 untouched, r10..r14 restored, r15 back to entry sp, T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[15];
    let saved = [cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14]];
    cpu.r[15] = s.wrapping_sub(20); // push r14,r13,r12,r11,r10
    let d = cpu.r[4];

    cpu.fr[3] = 1.0; // fldi1 fr3
    let _ = mem.write_f32(d, cpu.fr[3]); // fmov fr3,@r4
    let g = mem.read_u32(0x8c0da9bc).unwrap(); // r6 = global pointer
    cpu.r[6] = g;
    let c0 = mem.read_f32(0x8c0da9c0).unwrap();
    let c1 = mem.read_f32(0x8c0da9c4).unwrap();
    let c2 = mem.read_f32(0x8c0da9c8).unwrap();
    let c3 = mem.read_f32(0x8c0da9cc).unwrap();

    let g0 = mem.read_f32(g).unwrap();
    let g1 = mem.read_f32(g + 4).unwrap();
    let g2 = mem.read_f32(g + 8).unwrap();
    let g3 = mem.read_f32(g + 12).unwrap();
    cpu.fr[2] = c0;

    let _ = mem.write_f32(d + 4, g0);
    let _ = mem.write_f32(d + 8, g1);
    let _ = mem.write_f32(d + 12, g2);
    let _ = mem.write_f32(d + 16, g3);
    let _ = mem.write_f32(d + 20, cpu.fr[2]);
    let _ = mem.write_f32(d + 24, c1);
    cpu.fr[1] = 0.0; // fldi0 fr1
    let _ = mem.write_f32(d + 28, cpu.fr[1]);
    let _ = mem.write_f32(d + 32, c2);
    let _ = mem.write_f32(d + 36, c3);
    let _ = mem.write_f32(d + 40, cpu.fr[3]);
    let _ = mem.write_u32(d + 44, 0); // mov #0,r0 ; mov.l r0,@(44,r4)
    let _ = mem.write_f32(d + 48, cpu.fr[2]);
    let _ = mem.write_f32(d + 52, g0);
    let _ = mem.write_f32(d + 56, g1);
    let _ = mem.write_f32(d + 60, g2);
    let _ = mem.write_f32(d + 64, g3);
    let _ = mem.write_f32(d + 68, g0);
    let _ = mem.write_f32(d + 72, g1);
    let _ = mem.write_f32(d + 76, g2);
    let _ = mem.write_f32(d + 80, g3);
    let _ = mem.write_f32(d + 84, g0);
    let _ = mem.write_f32(d + 88, g1);
    let _ = mem.write_f32(d + 92, g2);
    let _ = mem.write_f32(d + 96, g3);
    let _ = mem.write_f32(d + 100, cpu.fr[1]);
    let _ = mem.write_f32(d + 104, cpu.fr[1]);
    let _ = mem.write_f32(d + 108, cpu.fr[1]);

    cpu.fr[0] = g3; // last fmov load: @r1 with r1 = g+12
    cpu.r[0] = d;
    cpu.r[1] = d + 104;
    cpu.r[3] = g + 8;
    cpu.r[5] = g + 4;
    cpu.r[7] = d + 92;
    cpu.r[10] = saved[0];
    cpu.r[11] = saved[1];
    cpu.r[12] = saved[2];
    cpu.r[13] = saved[3];
    cpu.r[14] = saved[4];
    cpu.r[15] = s;
}
