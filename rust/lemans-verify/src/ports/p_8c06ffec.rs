//! 0x8c06ffec — FUN_8c06ffec: flat struct copy (253 instructions, straight line).
//!
//! CFG sketch (no branches at all — one basic block, 0x8c06ffec..0x8c0701e4):
//!
//! ```text
//! prologue: push r8..r14, r15 -= 40            (body r15 = r15_in - 68)
//!   dst[0..2]   = src[1..3]        (r7 = src+4)
//!   dst[3..6]   = src[0x2c..0x38]  (r0 = src+44)
//!   dst[7..18]  = src[0x80..0xac]  (r10 = src + pool[0x8c0701e8]=0x80, r9 = dst+28)
//!   dst[19..21] = src[0x10..0x18]  (r12 = src+16, r11 = dst+76)
//!   dst[22..24] = src[0x60..0x68]  (r14 = src+96, r13 = dst+88)
//!   fr0 = src[0x6c]; dst[0x19] = fr0          (fmov pair via stack slots)
//!   dst[0x1a..0x1c] = src[0x20..0x28]         (r6 = src+32, r7 = dst+104)
//!   dst[0x1d..0x1f] = src[0x3c..0x44]         (via stack slot sp+36, r8 = dst+116)
//!   dst[0x20..0x22] = src[0xcc..0xd4]         (slot sp+20, r1 = dst+0x80 from slot sp+16)
//!   dst[0x23..0x25] = src[0x4c..0x54]         (slot sp+28, r1 = dst+0x8c from slot sp+24)
//!   dst[0x26..0x3a] = src[0xd8..0xf8]         (r0 = src+0xd8, r3 = dst+0x98)
//!   fr0 = src[0x58]; dst[0x2f] = fr0          (r3 = dst+0xbc)
//!   fr0 = src[0x5c]; dst[0x30] = fr0          (r3 = dst+0xc0)
//!   dst[0x31..0x33] = src[0xb0..0xb8]         (r7 = src+0xb0, r8 = dst+0xc4)
//!   dst[0x34..0x36] = src[0xbc..0xc4]         (r4 = src+0xbc, r5 = dst+0xd0)
//! epilogue: r15 += 40, pop r8..r14, rts (delay: pop r14)  → r15 net unchanged
//! ```
//!
//! The literal pool at 0x8c0701e8..0x8c07020c (0x80,0xcc,0x8c,0xd8,0x98,0xbc,
//! 0xc0,0xb0,0xc4,0xd0) is read from the image, exactly like the original.
//!
//! Side effects the decompiled C does not show: r4 and r5 are clobbered
//! (r4 = src+0xbc, r5 = dst+0xd0); r2 is left = body r15 = r15_in-68; r0/r1/r3/r6
//! hold the last loaded src words (*(src+0xc4), *(src+0xbc), *(src+0xb4),
//! *(src+0xb0)); r8..r14 are restored by the epilogue so they keep their input
//! values; fr0 ends as *(src+0x5c); T/sr are never touched; the six fmovs are
//! 32-bit in this oracle regardless of FPSCR.SZ (spec .sz1 pins that down).

use sh4_oracle::{Cpu, Memory};

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).expect("8c06ffec: load out of range")
}

#[inline]
fn wr(mem: &mut Memory, addr: u32, v: u32) {
    mem.write_u32(addr, v).expect("8c06ffec: store out of range");
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r = cpu.r;
    // r15 during the body: 7 pushes + add #-40. The epilogue undoes it, so the
    // final r15 equals the input r15; stack slots live at body_r15+off.
    let sp = r[15].wrapping_sub(68);

    // Literal pool (read from the image like the `mov.l <pool>,rN` ops do).
    let k80 = rd(mem, 0x8c07_01e8);
    let kcc = rd(mem, 0x8c07_01ec);
    let k8c = rd(mem, 0x8c07_01f0);
    let kd8 = rd(mem, 0x8c07_01f4);
    let k98 = rd(mem, 0x8c07_01f8);
    let kbc = rd(mem, 0x8c07_01fc);
    let kc0 = rd(mem, 0x8c07_0200);
    let kb0 = rd(mem, 0x8c07_0204);
    let kc4 = rd(mem, 0x8c07_0208);
    let kd0 = rd(mem, 0x8c07_020c);

    let mut fr0 = cpu.fr[0].to_bits();

    r[7] = r[4].wrapping_add(4);
    r[6] = rd(mem, r[7]);
    r[0] = r[4];
    r[1] = rd(mem, r[7].wrapping_add(4));
    r[3] = r[5];
    r[9] = r[5];
    wr(mem, r[5], r[6]);
    r[2] = sp;
    wr(mem, r[5].wrapping_add(4), r[1]);
    r[12] = r[4];
    r[1] = rd(mem, r[7].wrapping_add(8));
    r[11] = r[5];
    r[0] = r[0].wrapping_add(44);
    wr(mem, r[5].wrapping_add(8), r[1]);
    r[14] = r[4];
    r[6] = rd(mem, r[0]);
    r[3] = r[3].wrapping_add(12);
    r[1] = rd(mem, r[0].wrapping_add(4));
    r[13] = r[5];
    r[7] = r[5];
    wr(mem, r[3], r[6]);
    r[10] = r[4];
    wr(mem, r[3].wrapping_add(4), r[1]);
    r[6] = r[4];
    r[1] = rd(mem, r[0].wrapping_add(8));
    r[9] = r[9].wrapping_add(28);
    r[8] = rd(mem, r[0].wrapping_add(12));
    r[2] = r[2].wrapping_add(36);
    r[12] = r[12].wrapping_add(16);
    r[0] = k80;
    r[11] = r[11].wrapping_add(76);
    r[14] = r[14].wrapping_add(96);
    wr(mem, r[3].wrapping_add(8), r[1]);
    r[10] = r[10].wrapping_add(r[0]);
    wr(mem, r[3].wrapping_add(12), r[8]);
    r[13] = r[13].wrapping_add(88);
    r[1] = rd(mem, r[10]);
    r[8] = r[5];
    r[0] = rd(mem, r[10].wrapping_add(4));
    r[6] = r[6].wrapping_add(32);
    r[7] = r[7].wrapping_add(104);
    wr(mem, r[9], r[1]);
    r[8] = r[8].wrapping_add(116);
    wr(mem, r[9].wrapping_add(4), r[0]);
    r[1] = r[4];
    r[0] = rd(mem, r[10].wrapping_add(8));
    r[1] = r[1].wrapping_add(108);
    r[3] = rd(mem, r[10].wrapping_add(12));
    wr(mem, sp.wrapping_add(12), r[1]);
    r[1] = r[5];
    wr(mem, r[9].wrapping_add(8), r[0]);
    r[1] = r[1].wrapping_add(100);
    wr(mem, sp.wrapping_add(8), r[1]);
    wr(mem, r[9].wrapping_add(12), r[3]);
    r[1] = r[4];
    r[0] = rd(mem, r[10].wrapping_add(16));
    r[1] = r[1].wrapping_add(60);
    r[3] = rd(mem, r[10].wrapping_add(20));
    wr(mem, r[2], r[1]);
    wr(mem, r[9].wrapping_add(16), r[0]);
    r[2] = sp;
    wr(mem, r[9].wrapping_add(20), r[3]);
    r[2] = r[2].wrapping_add(32);
    r[0] = kcc;
    r[1] = rd(mem, r[10].wrapping_add(24));
    wr(mem, r[2], r[1]);
    r[1] = r[4];
    r[2] = sp;
    r[1] = r[1].wrapping_add(r[0]);
    r[2] = r[2].wrapping_add(20);
    wr(mem, r[2], r[1]);
    r[2] = sp;
    r[0] = k80;
    r[1] = r[5];
    r[3] = rd(mem, r[10].wrapping_add(28));
    r[1] = r[1].wrapping_add(r[0]);
    r[2] = r[2].wrapping_add(16);
    wr(mem, r[2], r[1]);
    r[1] = r[4];
    r[2] = sp;
    r[1] = r[1].wrapping_add(76);
    r[2] = r[2].wrapping_add(28);
    wr(mem, r[2], r[1]);
    r[2] = sp;
    r[2] = r[2].wrapping_add(32);
    r[1] = rd(mem, r[2]);
    wr(mem, r[9].wrapping_add(24), r[1]);
    r[2] = sp;
    wr(mem, r[9].wrapping_add(28), r[3]);
    r[1] = r[5];
    r[0] = k8c;
    r[2] = r[2].wrapping_add(24);
    r[3] = rd(mem, r[10].wrapping_add(32));
    r[1] = r[1].wrapping_add(r[0]);
    r[0] = r[4];
    wr(mem, r[2], r[1]);
    r[1] = rd(mem, r[10].wrapping_add(36));
    r[2] = sp;
    r[2] = r[2].wrapping_add(36);
    wr(mem, sp, r[1]);
    wr(mem, r[9].wrapping_add(32), r[3]);
    r[1] = kd8;
    r[3] = r[5];
    r[0] = r[0].wrapping_add(r[1]);
    r[1] = rd(mem, sp);
    wr(mem, r[9].wrapping_add(36), r[1]);
    r[1] = rd(mem, r[10].wrapping_add(40));
    wr(mem, sp.wrapping_add(4), r[1]);
    r[1] = k98;
    r[10] = rd(mem, r[10].wrapping_add(44));
    r[3] = r[3].wrapping_add(r[1]);
    r[1] = rd(mem, sp.wrapping_add(4));
    wr(mem, r[9].wrapping_add(40), r[1]);
    wr(mem, r[9].wrapping_add(44), r[10]);
    r[9] = rd(mem, r[12]);
    r[1] = rd(mem, r[12].wrapping_add(4));
    wr(mem, r[11], r[9]);
    wr(mem, r[11].wrapping_add(4), r[1]);
    r[1] = rd(mem, r[12].wrapping_add(8));
    wr(mem, r[11].wrapping_add(8), r[1]);
    r[9] = rd(mem, r[14]);
    r[1] = rd(mem, r[14].wrapping_add(4));
    wr(mem, r[13], r[9]);
    wr(mem, r[13].wrapping_add(4), r[1]);
    r[1] = rd(mem, r[14].wrapping_add(8));
    wr(mem, r[13].wrapping_add(8), r[1]);
    r[1] = rd(mem, sp.wrapping_add(12));
    fr0 = rd(mem, r[1]);
    r[1] = rd(mem, sp.wrapping_add(8));
    wr(mem, r[1], fr0);
    r[9] = rd(mem, r[6]);
    r[1] = rd(mem, r[6].wrapping_add(4));
    wr(mem, r[7], r[9]);
    wr(mem, r[7].wrapping_add(4), r[1]);
    r[1] = rd(mem, r[6].wrapping_add(8));
    wr(mem, r[7].wrapping_add(8), r[1]);
    r[1] = rd(mem, r[2]);
    r[6] = rd(mem, r[1]);
    r[2] = sp;
    r[2] = r[2].wrapping_add(36);
    r[1] = rd(mem, r[1].wrapping_add(4));
    wr(mem, r[8], r[6]);
    wr(mem, r[8].wrapping_add(4), r[1]);
    r[1] = rd(mem, r[2]);
    r[1] = rd(mem, r[1].wrapping_add(8));
    r[2] = sp;
    r[2] = r[2].wrapping_add(20);
    wr(mem, r[8].wrapping_add(8), r[1]);
    r[1] = rd(mem, r[2]);
    r[2] = sp;
    r[7] = rd(mem, r[1]);
    r[2] = r[2].wrapping_add(16);
    r[6] = rd(mem, r[1].wrapping_add(4));
    r[1] = rd(mem, r[2]);
    wr(mem, r[1], r[7]);
    r[2] = sp;
    wr(mem, r[1].wrapping_add(4), r[6]);
    r[2] = r[2].wrapping_add(20);
    r[1] = rd(mem, r[2]);
    r[2] = sp;
    r[6] = rd(mem, r[1].wrapping_add(8));
    r[2] = r[2].wrapping_add(16);
    r[1] = rd(mem, r[2]);
    r[2] = sp;
    wr(mem, r[1].wrapping_add(8), r[6]);
    r[2] = r[2].wrapping_add(28);
    r[1] = rd(mem, r[2]);
    r[2] = sp;
    r[7] = rd(mem, r[1]);
    r[2] = r[2].wrapping_add(24);
    r[6] = rd(mem, r[1].wrapping_add(4));
    r[1] = rd(mem, r[2]);
    wr(mem, r[1], r[7]);
    r[2] = sp;
    wr(mem, r[1].wrapping_add(4), r[6]);
    r[2] = r[2].wrapping_add(28);
    r[1] = rd(mem, r[2]);
    r[2] = sp;
    r[6] = rd(mem, r[1].wrapping_add(8));
    r[2] = r[2].wrapping_add(24);
    r[1] = rd(mem, r[2]);
    wr(mem, r[1].wrapping_add(8), r[6]);
    r[6] = rd(mem, r[0]);
    r[1] = rd(mem, r[0].wrapping_add(4));
    wr(mem, r[3], r[6]);
    wr(mem, r[3].wrapping_add(4), r[1]);
    r[6] = rd(mem, r[0].wrapping_add(8));
    r[1] = rd(mem, r[0].wrapping_add(12));
    wr(mem, r[3].wrapping_add(8), r[6]);
    wr(mem, r[3].wrapping_add(12), r[1]);
    r[6] = rd(mem, r[0].wrapping_add(16));
    r[1] = rd(mem, r[0].wrapping_add(20));
    wr(mem, r[3].wrapping_add(16), r[6]);
    wr(mem, r[3].wrapping_add(20), r[1]);
    r[6] = rd(mem, r[0].wrapping_add(24));
    r[1] = rd(mem, r[0].wrapping_add(28));
    wr(mem, r[3].wrapping_add(24), r[6]);
    wr(mem, r[3].wrapping_add(28), r[1]);
    r[0] = rd(mem, r[0].wrapping_add(32));
    wr(mem, r[3].wrapping_add(32), r[0]);
    r[0] = r[4];
    r[0] = r[0].wrapping_add(88);
    fr0 = rd(mem, r[0]);
    r[1] = r[4];
    r[3] = r[5];
    r[0] = kbc;
    r[7] = r[4];
    r[1] = r[1].wrapping_add(92);
    r[3] = r[3].wrapping_add(r[0]);
    wr(mem, r[3], fr0);
    r[8] = r[5];
    fr0 = rd(mem, r[1]);
    r[3] = r[5];
    r[6] = kc0;
    r[4] = r[4].wrapping_add(r[0]);
    r[1] = kb0;
    r[3] = r[3].wrapping_add(r[6]);
    wr(mem, r[3], fr0);
    r[7] = r[7].wrapping_add(r[1]);
    r[1] = kc4;
    r[6] = rd(mem, r[7]);
    r[8] = r[8].wrapping_add(r[1]);
    r[1] = kd0;
    r[3] = rd(mem, r[7].wrapping_add(4));
    r[5] = r[5].wrapping_add(r[1]);
    wr(mem, r[8], r[6]);
    wr(mem, r[8].wrapping_add(4), r[3]);
    r[0] = rd(mem, r[7].wrapping_add(8));
    wr(mem, r[8].wrapping_add(8), r[0]);
    r[1] = rd(mem, r[4]);
    r[0] = rd(mem, r[4].wrapping_add(4));
    wr(mem, r[5], r[1]);
    wr(mem, r[5].wrapping_add(4), r[0]);
    r[0] = rd(mem, r[4].wrapping_add(8));
    wr(mem, r[5].wrapping_add(8), r[0]);

    // Epilogue: r15 net unchanged, r8..r14 restored by the pops to their input
    // values; r0..r7 keep the body values.
    for i in 8..=14 {
        r[i] = cpu.r[i];
    }
    cpu.r = r;
    cpu.fr[0] = f32::from_bits(fr0);
}
