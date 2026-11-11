//! 0x8c05a828 — build a 4x4 float matrix at r4 and transpose it in place.
//!
//! Prologue: pushes r10..r14, reserves 64 bytes, then fills a local array
//! at sp+0..63 from fr4/fr5/fr6/fr7 and the literal 2.0f at 0x8c05a94c:
//!   a = 2.0 / fr7 ; loc[12]=a, loc[13..15]=0
//!   fr0 = 0 ; fr3 = 1
//!   fr5 = fr5 - fr4 ; fr2 = 1.0 ; fr1 = a / fr6 ; fr2 = fr2 / fr5
//!   fr4 = -fr4 * fr2
//!   loc[8..11] = {0, a/fr6, 0, 0}, loc[4..7] = {0, 0, fr2, fr4},
//!   loc[0..3] = {0, 0, 0, 1.0}
//! and copies it into [r4..r4+63] in permuted row order:
//!   row0 = loc[12..15], row1 = loc[8..11], row2 = loc[4..7], row3 = loc[0..3].
//!
//! Then a triangular swap loop transposes the matrix: for k = 1..=3
//! (r1), for j = 0..k-1: swap f32 at r4+16*(k-1)+4*j with r4+16*j+4*(k-1).
//! Each swap stores [r7] = fr0 first, then [r6] = fr1 in the bt.s delay
//! slot — write order matters.
//!
//! Leftovers the C does not show: r0 = 4, r1 = 4, r2 = sp+12, r3 = r4+64,
//! r5 = 16, r6 = r4+44, r7 = r4+56 (loop temporaries), r10..r14 restored
//! by the epilogue pops, r15 net unchanged, T = 0 (final cmp/gt 4>4),
//! fr0 = last swapped value, fr1 = 0.0 (last source load), fr2 = 1/(fr5-fr4),
//! fr3 = 1.0, fr4 = -fr4*(1/(fr5-fr4)), fr5 = fr5-fr4.
//! sp below is the body stack pointer (entry r15 - 84); all sp-relative
//! stores land in the synthetic stack window and are excluded from the
//! diff, so they are modelled in a local array instead.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sp = cpu.r[15].wrapping_sub(84); // 5 pushes + add #-64,r15
    let dst = cpu.r[4];
    let mut loc = [0u32; 16]; // dwords at sp+0..63

    // 0x8c05a834..0x8c05a85a
    cpu.fr[1] = mem.read_f32(0x8c05a94c).unwrap(); // mova 0x8c05a94c,r0 ; fmov @r0,fr1
    cpu.fr[1] = cpu.fr[1] / cpu.fr[7]; // fdiv fr7,fr1
    loc[12] = cpu.fr[1].to_bits(); // fmov fr1,@(sp+48)
    cpu.fr[0] = 0.0; // fldi0 fr0
    loc[13] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+52)
    loc[14] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+56)
    cpu.fr[3] = 1.0; // fldi1 fr3
    loc[15] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+60)

    // 0x8c05a85c..0x8c05a89c
    let mut r5 = loc[12]; // mov.l @(sp+48),r5
    let mut r3 = loc[13]; // mov.l @(sp+52),r3
    cpu.fr[5] = cpu.fr[5] - cpu.fr[4]; // fsub fr4,fr5
    cpu.fr[2] = cpu.fr[3]; // fmov fr3,fr2
    let _ = mem.write_u32(dst, r5); // mov.l r5,@r4
    let _ = mem.write_u32(dst.wrapping_add(4), r3); // mov.l r3,@(4,r4)
    r5 = loc[14]; // mov.l @(sp+56),r5
    r3 = loc[15]; // mov.l @(sp+60),r3
    cpu.fr[1] = cpu.fr[1] / cpu.fr[6]; // fdiv fr6,fr1
    let _ = mem.write_u32(dst.wrapping_add(8), r5); // mov.l r5,@(8,r4)
    let r1 = 0u32; // mov #0,r1
    let _ = mem.write_u32(dst.wrapping_add(12), r3); // mov.l r3,@(12,r4)
    loc[8] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+32)
    loc[9] = cpu.fr[1].to_bits(); // fmov fr1,@(sp+36)
    cpu.fr[2] = cpu.fr[2] / cpu.fr[5]; // fdiv fr5,fr2
    loc[10] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+40)
    cpu.fr[4] = -cpu.fr[4]; // fneg fr4
    loc[11] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+44)
    cpu.fr[4] = cpu.fr[4] * cpu.fr[2]; // fmul fr2,fr4

    // 0x8c05a89e..0x8c05a8bc: row1 <- loc[8..11]
    let mut r11 = loc[8];
    let mut r6 = loc[9];
    loc[4] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+16)
    let mut r0 = dst.wrapping_add(16); // mov r4,r0 ; add #16,r0
    let _ = mem.write_u32(r0, r11); // mov.l r11,@r0        -> [r4+16]
    let r12 = dst.wrapping_add(32); // mov r4,r12 ; add #32,r12
    let _ = mem.write_u32(r0.wrapping_add(4), r6); // mov.l r6,@(4,r0) -> [r4+20]
    r11 = loc[10];
    let mut r14 = dst.wrapping_add(48); // mov r4,r14 ; add #48,r14
    r6 = loc[11];
    let _ = mem.write_u32(r0.wrapping_add(8), r11); // mov.l r11,@(8,r0)  -> [r4+24]
    let _ = mem.write_u32(r0.wrapping_add(12), r6); // mov.l r6,@(12,r0)  -> [r4+28]

    // 0x8c05a8be..0x8c05a8d0: loc[4..7] = {0, 0, fr2, fr4}
    loc[4] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+16)
    loc[5] = cpu.fr[0].to_bits(); // fmov fr0,@(sp+20)
    loc[6] = cpu.fr[2].to_bits(); // fmov fr2,@(sp+24)
    loc[7] = cpu.fr[4].to_bits(); // fmov fr4,@(sp+28)

    // 0x8c05a8d2..0x8c05a8e4: row2 <- loc[4..7]
    r6 = loc[4];
    let mut r0b = loc[5];
    let _ = mem.write_u32(r12, r6); // mov.l r6,@r12          -> [r4+32]
    let _ = mem.write_u32(r12.wrapping_add(4), r0b); // mov.l r0,@(4,r12) -> [r4+36]
    r6 = loc[6];
    r0b = loc[7];
    let _ = mem.write_u32(r12.wrapping_add(8), r6); // -> [r4+40]
    let _ = mem.write_u32(r12.wrapping_add(12), r0b); // -> [r4+44]

    // 0x8c05a8e6..0x8c05a8f8: loc[0..3] = {0, 0, 0, fr3}
    loc[0] = cpu.fr[0].to_bits();
    loc[1] = cpu.fr[0].to_bits();
    loc[2] = cpu.fr[0].to_bits();
    loc[3] = cpu.fr[3].to_bits(); // fmov fr3,@(sp+12)

    // 0x8c05a8fa..0x8c05a908: row3 <- loc[0..3]
    r6 = loc[0];
    r0b = loc[1];
    let _ = mem.write_u32(r14, r6); // -> [r4+48]
    let _ = mem.write_u32(r14.wrapping_add(4), r0b); // -> [r4+52]
    r6 = loc[2];
    r0b = loc[3];
    let _ = mem.write_u32(r14.wrapping_add(8), r6); // -> [r4+56]
    let _ = mem.write_u32(r14.wrapping_add(12), r0b); // -> [r4+60]

    // 0x8c05a90a..: transpose loop. r14 = r1 = 0, r0 = 4, r3 = r4, r5 = 0.
    let mut r1 = r1;
    let mut r3 = dst;
    let mut r5 = 0u32;
    let r14 = r1; // mov r1,r14
    let r0c = 4u32; // mov #4,r0
    let mut r6 = 0u32;
    let mut r7 = 0u32;
    let mut r10 = 0u32;
    let mut r11 = 0u32;
    let mut r12l = 0u32;
    loop {
        // 0x8c05a90e: cmp/pl r1 ; bf.s 0x8c05a932 (delay: mov r14,r10)
        r10 = r14;
        if (r1 as i32) <= 0 {
            // branch taken -> 0x8c05a932
            r1 = r1.wrapping_add(1); // add #1,r1
            let t = (r0c as i32) > (r1 as i32); // cmp/gt r1,r0
            cpu.set_t(t);
            r3 = r3.wrapping_add(16); // add #16,r3
            r5 = r5.wrapping_add(4); // bt.s delay slot: add #4,r5
            if t {
                continue; // bt.s 0x8c05a90e
            }
            break;
        }
        // 0x8c05a914: mov r14,r11 ; mov r4,r12
        r11 = r14;
        r12l = dst;
        // inner loop head 0x8c05a918
        loop {
            r6 = r12l; // mov r12,r6
            r7 = r3; // mov r3,r7
            r7 = r7.wrapping_add(r11); // add r11,r7
            r6 = r6.wrapping_add(r5); // add r5,r6
            cpu.fr[1] = mem.read_f32(r7).unwrap(); // fmov @r7,fr1
            r10 = r10.wrapping_add(1); // add #1,r10
            cpu.fr[0] = mem.read_f32(r6).unwrap(); // fmov @r6,fr0
            let t = (r1 as i32) > (r10 as i32); // cmp/gt r10,r1
            cpu.set_t(t);
            r11 = r11.wrapping_add(4); // add #4,r11
            let _ = mem.write_f32(r7, cpu.fr[0]); // fmov fr0,@r7
            r12l = r12l.wrapping_add(16); // add #16,r12
            let _ = mem.write_f32(r6, cpu.fr[1]); // bt.s delay slot: fmov fr1,@r6
            if t {
                continue; // bt.s 0x8c05a918
            }
            break;
        }
        // 0x8c05a932: add #1,r1 ; cmp/gt r1,r0 ; add #16,r3 ; bt.s 0x8c05a90e (delay add #4,r5)
        r1 = r1.wrapping_add(1);
        let t = (r0c as i32) > (r1 as i32);
        cpu.set_t(t);
        r3 = r3.wrapping_add(16);
        r5 = r5.wrapping_add(4);
        if t {
            continue;
        }
        break;
    }

    // Final architectural state.
    cpu.r[0] = r0c; // 4
    cpu.r[1] = r1; // 4
    cpu.r[2] = sp.wrapping_add(12); // last mov #12,r2 ; add r15,r2
    cpu.r[3] = r3; // r4+64
    cpu.r[4] = dst;
    cpu.r[5] = r5; // 16
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    // r10..r14 are never modified by this port (the original clobbers them in
    // the loop but restores them with the epilogue pops), so they already
    // hold the entry values; r15 is net unchanged by push/add/pop.
    cpu.set_t(false); // final cmp/gt r1,r0 (4 > 4) leaves T = 0
}
