//! 0x8c0c5714 — construct an object from two integer pairs, two FPU inputs
//! and a third pair loaded through a pointer.
//!
//! ```text
//! push fr15,fr14,fr13 ; fr14=fr5 ; fr13=fr4
//! push entry r14,r13,r12,r11,pr ; add #-4,r15
//! r11 = r4 (object) ; fr15 = *r7
//! call 0x8c05d378(object)
//! *object = 0x8c1eb940
//! call 0x8c05d274(object, 1, 1)
//! *object = 0x8c1e8b54
//! [object+56] = r5[0] ; [object+60] = r5[1]
//! [object+64] = r6[0] ; [object+68] = r6[1]
//! *object = 0x8c1eb828
//! [object+72] = fr4 (entry) ; [object+76] = fr5 (entry) ; [object+80] = *r7
//! fr0=[object+72], fr1=[object+80] ; fcmp/gt fr1,fr0 ; if fr0>fr1 fr1=fr0
//! [object+80] = fr1
//! fr0=[object+76], fr1=[object+80] ; fcmp/gt fr0,fr1 ; if fr1>fr0 fr1=fr0
//! [object+80] = fr1
//! *object = 0x8c1eb7dc ; [object+84] = r7
//! return object (r0), r1=object+80, r3=object+72, r7=object+64
//! restore r11..r14, fr13..fr15, pr; T is the last fcmp/gt
//! ```
//!
//! `fcmp/gt FRm,FRn` sets T for `FRn > FRm`; the `bt` arm copies FRm over FRn,
//! so the first block picks the larger of object+72 and object+80, and the
//! second picks the smaller of object+76 and that result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let entry_pr = cpu.pr;

    // 0x8c0c5714: fmov fr15,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.fr[15].to_bits());
    // 0x8c0c5716: fmov fr14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.fr[14].to_bits());
    // 0x8c0c5718: fmov fr13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.fr[13].to_bits());
    // 0x8c0c571a: fmov fr5,fr14
    cpu.fr[14] = cpu.fr[5];
    // 0x8c0c571c: mov.l r14,@-r15  (entry r14)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0c571e: fmov fr4,fr13
    cpu.fr[13] = cpu.fr[4];
    // 0x8c0c5720: mov.l r13,@-r15  (entry r13)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0c5722: mov r7,r14
    cpu.r[14] = cpu.r[7];
    // 0x8c0c5724: mov.l r12,@-r15  (entry r12)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0c5726: mov r6,r13
    cpu.r[13] = cpu.r[6];
    // 0x8c0c5728: mov.l r11,@-r15  (entry r11)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c0c572a: mov r5,r12
    cpu.r[12] = cpu.r[5];
    // 0x8c0c572c: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], entry_pr);
    // 0x8c0c572e: add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c0c5730: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c0c5732: fmov @r14,fr15
    cpu.fr[15] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    // 0x8c0c5734: mov.l 0x8c0c57c8,r0  (0x8c05d378)
    cpu.r[0] = mem.read_u32(0x8c0c57c8).unwrap_or(0);
    // 0x8c0c5736: mov.l r11,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c0c5738: jsr @r0 / 0x8c0c573a: mov.l @r15,r4 (delay slot)
    cpu.pr = 0x8c0c573c;
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    crate::registry::p_8c05d378::run(cpu, mem);

    // 0x8c0c573c: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c0c573e: mov #1,r6
    cpu.r[6] = 1;
    // 0x8c0c5740: mov.l 0x8c0c57cc,r0  (0x8c1eb940)
    cpu.r[0] = mem.read_u32(0x8c0c57cc).unwrap_or(0);
    // 0x8c0c5742: mov r6,r5
    cpu.r[5] = cpu.r[6];
    // 0x8c0c5744: mov.l r0,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    // 0x8c0c5746: mov.l 0x8c0c57d0,r0  (0x8c05d274)
    cpu.r[0] = mem.read_u32(0x8c0c57d0).unwrap_or(0);
    // 0x8c0c5748: jsr @r0 / 0x8c0c574a: mov.l @r15,r4 (delay slot)
    cpu.pr = 0x8c0c574c;
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    crate::registry::p_8c05d274::run(cpu, mem);

    // 0x8c0c574c: mov.l 0x8c0c57d4,r0  (0x8c1e8b54)
    cpu.r[0] = mem.read_u32(0x8c0c57d4).unwrap_or(0);
    // 0x8c0c574e: mov r11,r4
    cpu.r[4] = cpu.r[11];
    // 0x8c0c5750: mov r11,r7
    cpu.r[7] = cpu.r[11];
    // 0x8c0c5752: mov.l r0,@r11
    let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
    // 0x8c0c5754: mov r11,r3
    cpu.r[3] = cpu.r[11];
    // 0x8c0c5756: mov.l @r12,r1
    cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    // 0x8c0c5758: add #56,r4
    cpu.r[4] = cpu.r[4].wrapping_add(56);
    // 0x8c0c575a: mov.l @(4,r12),r0
    cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
    // 0x8c0c575c: add #64,r7
    cpu.r[7] = cpu.r[7].wrapping_add(64);
    // 0x8c0c575e: add #72,r3
    cpu.r[3] = cpu.r[3].wrapping_add(72);
    // 0x8c0c5760: mov.l r1,@r4
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    // 0x8c0c5762: mov.l r0,@(4,r4)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
    // 0x8c0c5764: mov r11,r1
    cpu.r[1] = cpu.r[11];
    // 0x8c0c5766: mov.l @r13,r6
    cpu.r[6] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c0c5768: mov r11,r0
    cpu.r[0] = cpu.r[11];
    // 0x8c0c576a: mov.l @(4,r13),r5
    cpu.r[5] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    // 0x8c0c576c: add #76,r1
    cpu.r[1] = cpu.r[1].wrapping_add(76);
    // 0x8c0c576e: mov.l 0x8c0c57d8,r4  (0x8c1eb828)
    cpu.r[4] = mem.read_u32(0x8c0c57d8).unwrap_or(0);
    // 0x8c0c5770: add #80,r0
    cpu.r[0] = cpu.r[0].wrapping_add(80);
    // 0x8c0c5772: mov.l r6,@r7
    let _ = mem.write_u32(cpu.r[7], cpu.r[6]);
    // 0x8c0c5774: mov.l r5,@(4,r7)
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[5]);
    // 0x8c0c5776: mov.l r4,@r11
    let _ = mem.write_u32(cpu.r[11], cpu.r[4]);
    // 0x8c0c5778: fmov fr13,@r3
    let _ = mem.write_f32(cpu.r[3], cpu.fr[13]);
    // 0x8c0c577a: fmov fr14,@r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[14]);
    // 0x8c0c577c: fmov fr15,@r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);

    // 0x8c0c577e: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    // 0x8c0c5780: fmov @r0,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0c5782: fcmp/gt fr1,fr0  (T = fr0 > fr1)
    cpu.set_t(cpu.fr[0] > cpu.fr[1]);
    // 0x8c0c5784: bt 0x8c0c578a / 0x8c0c5786: bra 0x8c0c578c
    if cpu.t() {
        // 0x8c0c578a: fmov fr0,fr1
        cpu.fr[1] = cpu.fr[0];
    }
    // 0x8c0c578c: mov r11,r1
    cpu.r[1] = cpu.r[11];
    // 0x8c0c578e: add #80,r1
    cpu.r[1] = cpu.r[1].wrapping_add(80);
    // 0x8c0c5790: mov r11,r0
    cpu.r[0] = cpu.r[11];
    // 0x8c0c5792: fmov fr1,@r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    // 0x8c0c5794: add #76,r0
    cpu.r[0] = cpu.r[0].wrapping_add(76);
    // 0x8c0c5796: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0c5798: fmov @r1,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    // 0x8c0c579a: fcmp/gt fr0,fr1  (T = fr1 > fr0)
    cpu.set_t(cpu.fr[1] > cpu.fr[0]);
    // 0x8c0c579c: bt 0x8c0c57a2 / 0x8c0c579e: bra 0x8c0c57a4
    if cpu.t() {
        // 0x8c0c57a2: fmov fr0,fr1
        cpu.fr[1] = cpu.fr[0];
    }
    // 0x8c0c57a4: mov r11,r0
    cpu.r[0] = cpu.r[11];
    // 0x8c0c57a6: add #80,r0
    cpu.r[0] = cpu.r[0].wrapping_add(80);
    // 0x8c0c57a8: fmov fr1,@r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);

    // 0x8c0c57aa: mov.l 0x8c0c57dc,r0  (0x8c1eb7dc)
    cpu.r[0] = mem.read_u32(0x8c0c57dc).unwrap_or(0);
    // 0x8c0c57ac: mov.l r0,@r11
    let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
    // 0x8c0c57ae: mov #84,r0
    cpu.r[0] = 84;
    // 0x8c0c57b0: mov.l r14,@(r0,r11)
    let _ = mem.write_u32(cpu.r[11].wrapping_add(cpu.r[0]), cpu.r[14]);
    // 0x8c0c57b2: mov r11,r0
    cpu.r[0] = cpu.r[11];

    // epilogue
    // 0x8c0c57b4: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57b6: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57b8: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57ba: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57bc: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57be: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57c0: fmov @r15+,fr13
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57c2: fmov @r15+,fr14
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c57c4: rts / 0x8c0c57c6: fmov @r15+,fr15 (delay slot)
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
