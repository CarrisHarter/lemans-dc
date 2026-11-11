//! 0x8c0b2f78 — build a 3x3 matrix in `[r4]` from the axis vector in `[r5]`.
//!
//! Loads a=[r5], b=[r5+4], c=[r5+8], d=[r5+12]; every `fr` below 12 is
//! clobbered. fr13/fr14/fr15 and r14 are saved/restored through r15, so the
//! port leaves them at their entry values. Final integer registers are the
//! running pointers: r0=r4+12, r1=r4+8, r3=r4+24, r5=r4+20, r6=r4+28,
//! r7=r4+32, r4=r4+4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    let dst = cpu.r[4];
    let a = mem.read_f32(src).unwrap_or(0.0);
    let b = mem.read_f32(src + 4).unwrap_or(0.0);
    let c = mem.read_f32(src + 8).unwrap_or(0.0);
    let d = mem.read_f32(src + 12).unwrap_or(0.0);

    let c2 = c + c; // fr15 = fr0+fr0
    let b2 = b + b; // fr14 = fr13+fr13
    let d2 = d + d; // fr10 = fr11+fr11
    let fr9 = c2 * c; // fr9  = fr15*fr0
    let fr8 = d2 * d; // fr8  = fr10*fr11
    let fr1 = 1.0 - fr9 - fr8; // fr1
    let fr14 = b2 * b; // fr14 = fr14*fr13
    let fr7 = b2 * c; // fr7  = fr14*fr0
    let fr2 = 1.0 - fr14; // fr2
    let m16 = fr2 - fr8; // fr0  = fr2-fr8
    let fr5 = c2 * d; // fr5  = fr15*fr11
    let m32 = fr2 - fr9; // fr2  = fr2-fr9
    let fr3 = b2 * a; // fr3  = fr14*fr4
    let m28 = fr5 - fr3; // fr0  = fr5-fr3
    let fr6 = b2 * d; // fr6  = fr14*fr11
    let m20 = fr5 + fr3; // fr5  = fr5+fr3
    let fr15 = c2 * a; // fr15 = fr15*fr4
    let m24 = fr6 + fr15; // fr0  = fr6+fr15
    let m8 = fr6 - fr15; // fr6  = fr6-fr15
    let fr10 = d2 * a; // fr10 = fr10*fr4
    let m12 = fr7 - fr10; // fr0  = fr7-fr10
    let m4 = fr7 + fr10; // fr7  = fr7+fr10

    let _ = mem.write_f32(dst, fr1);
    let _ = mem.write_f32(dst + 16, m16);
    let _ = mem.write_f32(dst + 32, m32);
    let _ = mem.write_f32(dst + 28, m28);
    let _ = mem.write_f32(dst + 20, m20);
    let _ = mem.write_f32(dst + 24, m24);
    let _ = mem.write_f32(dst + 8, m8);
    let _ = mem.write_f32(dst + 12, m12);
    let _ = mem.write_f32(dst + 4, m4);

    cpu.fr[0] = m12;
    cpu.fr[1] = fr1;
    cpu.fr[2] = m32;
    cpu.fr[3] = fr3;
    cpu.fr[4] = a;
    cpu.fr[5] = m20;
    cpu.fr[6] = m8;
    cpu.fr[7] = m4;
    cpu.fr[8] = fr8;
    cpu.fr[9] = fr9;
    cpu.fr[10] = fr10;
    cpu.fr[11] = d;

    cpu.r[0] = dst + 12;
    cpu.r[1] = dst + 8;
    cpu.r[3] = dst + 24;
    cpu.r[4] = dst + 4;
    cpu.r[5] = dst + 20;
    cpu.r[6] = dst + 28;
    cpu.r[7] = dst + 32;
}
