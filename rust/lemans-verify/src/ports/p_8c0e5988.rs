//! 0x8c0e5988 — clip a segment against the view volume in single precision and,
//! if it survives, append a lit vertex record to the queue at *0x8c27ad10.
//!
//! CFG (no loops):
//!
//! ```text
//! prologue: push fr13/fr14/fr15 ; fr0=-1.0 ; fr3=10000.0
//! fr9=-fr9 ; fr11=fr9 ; fr6=-fr6 ; fr11-=fr6 ; fr2=-fr11 ; fr1=0 ; fr2=1
//! fr10=fr7-fr4 ; fr15=fr8-fr5
//! fcmp/gt fr1,fr2 (T = fr2>0 i.e. fr11<0) ; bf.s pathB (delay: r0=1)
//! pathA (fr11<0):  fr14=(fr6+1)/(-fr11)
//!   fr14>1   -> bra end (delay r0=r1=0)
//!   fr14>0   -> bra end (delay fr1=fr14)        [r0 stays 1]
//! pathB (fr11>=0): fr13=-fr11 ; if fr1>fr13:
//!   fr13=(fr6+1)/(-fr11) ; if fr1>fr13 -> bra end (delay r0=r1=0)
//!   else if fr2>fr13 -> bra end (delay fr2=fr13)
//!   end: fr14=fr6+1 ; if fr14>fr1 -> r0=r1=0
//! end (0x8c0e59fc): r0=extu.b(r0) ; tst r0,r0 ; bt skipA (0x8c0e5a86)
//! skipA body: fr14=0 ; fcmp/gt fr14,fr11 ; bf.s (delay r0=1)
//!   ... three sub-tests each ending in bra 0x8c0e5a4a with delay slots
//!   that either clear r0 (mov #0,r0) or capture fr0 into fr1/fr2 ...
//! 0x8c0e5a4a: r0=extu.b(r0) ; tst ; bt skipB (0x8c0e5a86)
//! skipB body: fr0=1 ; fcmp/gt fr2,fr0 ; bf.s (delay r1=1)
//!   ... scale fr2/fr7/fr8 by (fr11,fr10,fr15)+offsets, then fr1 similarly
//! 0x8c0e5a86: r0=extu.b(r1) ; tst r0,r0 ; fneg fr6 ; bt.s epilogue (delay fneg fr9)
//! output body: fr10=1/fr9 ; fr9=1/fr6 ; normalise fr7/fr8/fr4/fr5 ;
//!   |fr7*k-fr4*k| vs |fr8*k-fr5*k| fcmp/gt ; bf.s (delay fr10*=const)
//!   pick (+1) offsets ; then 12 queue stores via *0x8c27ad10 with pref hints
//! epilogue: pop fr13,fr14 ; rts ; pop fr15 (delay slot)
//! ```
//!
//! Side effects the C hides: r0/r1 are clip flags reused as extu.b temps,
//! r0 is overwritten by mova (0x8c0e5b60/64/68) then ends as the queue head
//! pointer 0x8c27ad10 on the output path, r1 ends as scratch (r3+4) on the
//! output path, r3/r4 are queue pointers, fr0..fr15 all churn (fr9/fr6 are
//! negated twice on the output path, net zero), T ends as the last fcmp/gt or
//! tst, and the `bf.s`/`bra` delay slots run whether or not the branch is taken.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: fmov fr15/fr14/fr13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);

    cpu.fr[0] = mem.read_f32(0x8C0E5B60).unwrap(); // -1.0 (mova 0x8c0e5b60,r0)
    cpu.fr[9] = -cpu.fr[9]; // fneg fr9
    cpu.fr[11] = cpu.fr[9]; // fmov fr9,fr11
    cpu.fr[6] = -cpu.fr[6]; // fneg fr6
    cpu.fr[11] = cpu.fr[11] - cpu.fr[6]; // fsub fr6,fr11
    cpu.fr[2] = cpu.fr[11]; // fmov fr11,fr2
    cpu.fr[3] = mem.read_f32(0x8C0E5B64).unwrap(); // 10000.0
    cpu.fr[10] = cpu.fr[7]; // fmov fr7,fr10
    cpu.fr[15] = cpu.fr[8]; // fmov fr8,fr15
    cpu.fr[2] = -cpu.fr[2]; // fneg fr2
    cpu.fr[1] = 0.0; // fldi0 fr1
    cpu.set_t(cpu.fr[2] > cpu.fr[1]); // fcmp/gt fr1,fr2 → T = fr2 > fr1
    cpu.fr[10] = cpu.fr[10] - cpu.fr[4]; // fsub fr4,fr10
    cpu.fr[15] = cpu.fr[15] - cpu.fr[5]; // fsub fr5,fr15
    cpu.fr[2] = 1.0; // fldi1 fr2
    cpu.r[1] = 0;
    cpu.r[0] = 1; // delay slot of bf.s 0x8c0e59d0 — always
    if cpu.t() {
        // fall-through (T==1, fr11 < 0): 8c0e59b6..8c0e59ce — all roads lead to 0x8c0e59fc
        cpu.fr[14] = cpu.fr[6];
        cpu.fr[13] = cpu.fr[11];
        cpu.fr[14] = cpu.fr[14] - cpu.fr[0]; // fr6 - (-1.0)
        cpu.fr[13] = -cpu.fr[13];
        cpu.fr[14] = cpu.fr[14] / cpu.fr[13]; // fdiv fr13,fr14
        cpu.set_t(cpu.fr[14] > cpu.fr[2]); // fcmp/gt fr2,fr14 → T = fr14 > fr2
        if cpu.t() {
            cpu.r[0] = cpu.r[1]; // delay of bra 0x8c0e59c4
        } else {
            cpu.set_t(cpu.fr[14] > cpu.fr[1]); // fcmp/gt fr1,fr14 → T = fr14 > fr1
            if cpu.t() {
                cpu.fr[1] = cpu.fr[14]; // delay of bra 0x8c0e59cc
            }
        }
    } else {
        // branch target (T==0, fr11 >= 0): 8c0e59d0..8c0e59fa
        cpu.fr[13] = cpu.fr[11];
        cpu.fr[13] = -cpu.fr[13];
        cpu.set_t(cpu.fr[1] > cpu.fr[13]); // fcmp/gt fr13,fr1 → T = fr1 > fr13
        if cpu.t() {
            // 8c0e59d8..8c0e59f0 — every bra here goes straight to 0x8c0e59fc,
            // skipping 0x8c0e59f2
            cpu.fr[13] = cpu.fr[6];
            cpu.fr[14] = cpu.fr[11];
            cpu.fr[13] = cpu.fr[13] - cpu.fr[0];
            cpu.fr[14] = -cpu.fr[14];
            cpu.fr[13] = cpu.fr[13] / cpu.fr[14]; // fdiv fr14,fr13
            cpu.set_t(cpu.fr[1] > cpu.fr[13]); // fcmp/gt fr13,fr1
            if cpu.t() {
                cpu.r[0] = cpu.r[1]; // delay of bra 0x8c0e59e6
            } else {
                cpu.set_t(cpu.fr[2] > cpu.fr[13]); // fcmp/gt fr13,fr2 → T = fr2 > fr13
                if cpu.t() {
                    cpu.fr[2] = cpu.fr[13]; // delay of bra 0x8c0e59ee
                }
            }
        } else {
            // 8c0e59f2 (bf 0x8c0e59d6 taken)
            cpu.fr[14] = cpu.fr[6];
            cpu.fr[14] = cpu.fr[14] - cpu.fr[0];
            cpu.set_t(cpu.fr[14] > cpu.fr[1]); // fcmp/gt fr1,fr14 → T = fr14 > fr1
            if cpu.t() {
                cpu.r[0] = cpu.r[1]; // 8c0e59fa
            }
        }
    }
    // 8c0e59fc
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        // block A: 8c0e5a02..8c0e5a48
        cpu.fr[14] = 0.0; // fldi0 fr14
        cpu.set_t(cpu.fr[11] > cpu.fr[14]); // fcmp/gt fr14,fr11 → T = fr11 > fr14
        cpu.r[0] = 1; // delay slot of bf.s 0x8c0e5a22 — always
        if cpu.t() {
            // 8c0e5a0a..8c0e5a20
            cpu.fr[0] = cpu.fr[6];
            cpu.fr[0] = -cpu.fr[0];
            cpu.fr[0] = cpu.fr[0] - cpu.fr[3];
            cpu.fr[0] = cpu.fr[0] / cpu.fr[11]; // fdiv fr11,fr0
            cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt fr2,fr0 → T = fr0 > fr2
            if cpu.t() {
                cpu.r[0] = 0; // delay of bra 0x8c0e5a16
            } else {
                cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0 → T = fr0 > fr1
                if cpu.t() {
                    cpu.fr[1] = cpu.fr[0]; // delay of bra 0x8c0e5a1e
                }
            }
        }
        // 8c0e5a22
        cpu.set_t(cpu.fr[14] > cpu.fr[11]); // fcmp/gt fr11,fr14 → T = fr14 > fr11
        if cpu.t() {
            // 8c0e5a26..8c0e5a3c
            cpu.fr[0] = cpu.fr[6];
            cpu.fr[0] = -cpu.fr[0];
            cpu.fr[0] = cpu.fr[0] - cpu.fr[3];
            cpu.fr[0] = cpu.fr[0] / cpu.fr[11];
            cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 → T = fr1 > fr0
            if cpu.t() {
                cpu.r[0] = 0; // delay of bra 0x8c0e5a32
            } else {
                cpu.set_t(cpu.fr[2] > cpu.fr[0]); // fcmp/gt fr0,fr2 → T = fr2 > fr0
                if cpu.t() {
                    cpu.fr[2] = cpu.fr[0]; // delay of bra 0x8c0e5a3a
                }
            }
        }
        // 8c0e5a3e
        cpu.fr[0] = cpu.fr[6];
        cpu.fr[0] = -cpu.fr[0];
        cpu.fr[0] = cpu.fr[0] - cpu.fr[3];
        cpu.set_t(cpu.fr[0] > cpu.fr[14]); // fcmp/gt fr14,fr0 → T = fr0 > fr14
        if cpu.t() {
            cpu.r[0] = 0; // 8c0e5a48
        }
        // 8c0e5a4a
        cpu.r[0] &= 0xFF;
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            // block B: 8c0e5a50..8c0e5a84
            cpu.fr[0] = 1.0; // fldi1 fr0
            cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt fr2,fr0 → T = fr0 > fr2
            cpu.r[1] = 1; // delay slot of bf.s 0x8c0e5a6a — always
            if cpu.t() {
                // 8c0e5a58..8c0e5a68
                cpu.fr[7] = cpu.fr[2];
                cpu.fr[8] = cpu.fr[2];
                cpu.fr[2] = cpu.fr[2] * cpu.fr[11]; // fmul fr11,fr2
                cpu.fr[7] = cpu.fr[7] * cpu.fr[10];
                cpu.fr[8] = cpu.fr[8] * cpu.fr[15];
                cpu.fr[2] = cpu.fr[2] + cpu.fr[6];
                cpu.fr[9] = cpu.fr[2];
                cpu.fr[7] = cpu.fr[7] + cpu.fr[4];
                cpu.fr[8] = cpu.fr[8] + cpu.fr[5];
            }
            // 8c0e5a6a
            cpu.fr[0] = 0.0; // fldi0 fr0
            cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 → T = fr1 > fr0
            if cpu.t() {
                // 8c0e5a70..8c0e5a84
                cpu.fr[2] = cpu.fr[1];
                cpu.fr[0] = cpu.fr[1];
                cpu.fr[2] = cpu.fr[2] * cpu.fr[10];
                cpu.fr[0] = cpu.fr[0] * cpu.fr[15];
                cpu.fr[1] = cpu.fr[1] * cpu.fr[11];
                cpu.fr[2] = cpu.fr[2] + cpu.fr[4];
                cpu.fr[0] = cpu.fr[0] + cpu.fr[5];
                cpu.fr[1] = cpu.fr[1] + cpu.fr[6];
                cpu.fr[4] = cpu.fr[2];
                cpu.fr[5] = cpu.fr[0];
                cpu.fr[6] = cpu.fr[1];
            }
        }
    }
    // 8c0e5a86
    cpu.r[0] = cpu.r[1] & 0xFF; // extu.b r1,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.fr[6] = -cpu.fr[6]; // fneg fr6 — always (8c0e5a8a)
    cpu.fr[9] = -cpu.fr[9]; // delay slot of bt.s 0x8c0e5b56 — always
    if !cpu.t() {
        // output block: 8c0e5a90..8c0e5b54
        cpu.fr[3] = 1.0; // fldi1 fr3
        cpu.fr[10] = cpu.fr[3];
        cpu.fr[10] = cpu.fr[10] / cpu.fr[9]; // fdiv fr9,fr10
        cpu.fr[9] = cpu.fr[3];
        cpu.fr[9] = cpu.fr[9] / cpu.fr[6]; // fdiv fr6,fr9
        cpu.fr[7] = cpu.fr[7] * cpu.fr[10];
        cpu.fr[8] = cpu.fr[8] * cpu.fr[10];
        cpu.fr[2] = mem.read_f32(0x8C0E5B68).unwrap(); // mova 0x8c0e5b68,r0 ; fmov @r0,fr2
        cpu.fr[1] = cpu.fr[7];
        cpu.fr[4] = cpu.fr[4] * cpu.fr[9];
        cpu.fr[5] = cpu.fr[5] * cpu.fr[9];
        cpu.fr[0] = cpu.fr[8];
        cpu.fr[1] = cpu.fr[1] - cpu.fr[4];
        cpu.fr[0] = cpu.fr[0] - cpu.fr[5];
        cpu.fr[1] = cpu.fr[1].abs(); // fabs fr1
        cpu.fr[0] = cpu.fr[0].abs(); // fabs fr0
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 → T = fr1 > fr0
        cpu.fr[9] = cpu.fr[9] * cpu.fr[2];
        cpu.fr[10] = cpu.fr[10] * cpu.fr[2]; // delay slot of bf.s 0x8c0e5ac8 — always
        if cpu.t() {
            // 8c0e5aba..8c0e5ac6 (fr1 > fr0)
            cpu.fr[2] = cpu.fr[5];
            cpu.fr[0] = cpu.fr[8];
            cpu.fr[6] = cpu.fr[4];
            cpu.fr[1] = cpu.fr[7];
            cpu.fr[2] = cpu.fr[2] + cpu.fr[3];
            cpu.fr[0] = cpu.fr[0] + cpu.fr[3]; // delay of bra 0x8c0e5ac4
        } else {
            // 8c0e5ac8..8c0e5ad2
            cpu.fr[6] = cpu.fr[4];
            cpu.fr[1] = cpu.fr[7];
            cpu.fr[2] = cpu.fr[5];
            cpu.fr[0] = cpu.fr[8];
            cpu.fr[6] = cpu.fr[6] + cpu.fr[3];
            cpu.fr[1] = cpu.fr[1] + cpu.fr[3];
        }
        // 8c0e5ad4..8c0e5b54 — append the record to the queue at *0x8c27ad10
        cpu.r[0] = mem.read_u32(0x8C0E5B6C).unwrap(); // 0x8c27ad10
        cpu.r[1] = mem.read_u32(0x8C0E5B70).unwrap(); // 0xf0000000
        cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[3] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[3]);
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // header word
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[4]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[5]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[9]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[6]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[2]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[9]); // fr9 stored twice
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap();
        let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // no increment here
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        // pref @r3 — cache hint, no architectural effect
        cpu.r[3] = cpu.r[3].wrapping_add(4);
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[10]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[7]);
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[1] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[8]);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap();
        // pref @r1 — cache hint
        cpu.r[1] = cpu.r[1].wrapping_add(16);
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
    }
    // epilogue: 8c0e5b56..8c0e5b5c
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; delay slot: fmov @r15+,fr15
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
