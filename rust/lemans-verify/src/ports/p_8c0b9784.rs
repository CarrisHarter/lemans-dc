//! 0x8c0b9784 — weight accumulator. r4 -> byte flag + 4 entry pointers.
//! For each non-null entry with a zero stored weight, sum u16(entry->tab[i]+0x76)/10.0
//! over a 5-slot table row and add it to the entry's weight at +52, then into fr0.
//!
//! CFG (sketched before porting; loops are fixed-trip so they always terminate):
//! ```text
//! entry: push r8..r14 (all restored at exit)
//!   r0 = *r4 ; tst ; bt EARLY          ; *r4==0 -> fr0=0, T=1, return
//!   r13=r10=r1=r7=0 ; fr0..fr7=0 ; r3=5 ; r4+=4 ; r8=0x8c27a038 ; r14=0x8c27a0b8
//!   OUTER (r13=0..3, r10=0,4,8,12):      ; 0x8c0b97b8
//!     r9 = [r4+r10] ; tst ; bt NULLCHK
//!     fr2 = [r9+52] ; fcmp/eq fr7,fr2 ; bt NULLCHK   ; weight==0 -> NULLCHK
//!     fr0 += fr2 ; bra TAIL            ; delay slot fadd
//!   NULLCHK 0x8c0b97ce: tst r9,r9 ; bt NULL      ; r9==0 -> NULL branch
//!     cell = *r14 ; tst ; bt.s FR5     ; delay slot: r5 = [r9]
//!     r6 = r8 + 24*[r9] ; fr8 = fr6 ; INNER1 (r11=r7, r12=r7, 5 trips):
//!       e = [r6+r11] ; tst ; bt skip
//!       fr3 = float(extu.w [e+118]) / 10.0 ; fr8 += fr3   ; fpul = extu.w
//!       skip: r12++ ; cmp/gt r12,r3 ; bt.s INNER1 (delay r11+=4)
//!   FR5 0x8c0b9816: fr8 = fr5
//!   TAIL 0x8c0b9818: [r9+52] = fr8 ; fr2 = [r9+52]+fr0 ; fr0 = fr2 (delay) ; bra TAIL2
//!   NULL 0x8c0b982a: cell = *r14 ; tst ; bt FR1
//!     r5 = r8 + 24*r13 ; fr8 = fr4 ; INNER2 (r9=r1, r6=r1, 5 trips): same body
//!   FR1 0x8c0b986c: fr8 = fr1
//!   NULLTAIL 0x8c0b986e: fr0 += fr8
//!   TAIL2 0x8c0b9870: r13++ ; r0=4 ; cmp/gt r13,r0 ; bt.s OUTER (delay r10+=4)
//! EARLY 0x8c0b987e: fldi0 fr0 ; fallthrough to pops ; rts
//! ```
//!
//! Hidden side effects the decompiler does not show: r8..r14 are restored, but
//! r0=4 (loop exit) or 0 (early), r1=r7=0, r3=5, r4=arg+4, while r5/r6 end as
//! the last table base / mov.w value / inner2 counter; fr2 holds the last
//! reloaded weight+fr0, fr8 the last row sum, fpul the last extu.w, T=0 (loop)
//! or 1 (early). The only non-stack stores are the fr8 weights at entry+52.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap() as u32; // mov.b @r4,r0 ; extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.fr[0] = 0.0; // fldi0 fr0 at 0x8c0b987e
        return; // T=1; r8..r14 restored by the epilogue; r0=0
    }

    // r8..r14 are pushed on entry and popped at exit: keep them in locals.
    let mut lr13 = 0u32; // r13
    let mut lr10 = 0u32; // r10
    cpu.fr[0] = 0.0; // fldi0 fr0
    let lr8 = mem.read_u32(0x8c0b9890).unwrap(); // 0x8c27a038
    cpu.fr[4] = cpu.fr[0]; // fmov fr0,fr4
    let lr14 = mem.read_u32(0x8c0b9894).unwrap(); // 0x8c27a0b8
    cpu.r[1] = 0; // mov r13,r1
    cpu.fr[1] = cpu.fr[0]; // fmov fr0,fr1
    cpu.fr[6] = cpu.fr[0]; // fmov fr0,fr6
    cpu.r[7] = 0; // mov r13,r7
    cpu.fr[5] = cpu.fr[0]; // fmov fr0,fr5
    cpu.fr[7] = cpu.fr[0]; // fmov fr0,fr7
    cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
    cpu.r[3] = 5; // mov #5,r3

    loop {
        // OUTER head 0x8c0b97b8
        let lr9 = mem.read_u32(cpu.r[4].wrapping_add(lr10)).unwrap(); // mov.l @(r0,r4),r9
        cpu.set_t(lr9 == 0); // tst r9,r9
        if lr9 != 0 {
            cpu.r[0] = lr9; // mov r9,r0
            cpu.fr[2] = mem.read_f32(lr9.wrapping_add(52)).unwrap(); // fmov @r0,fr2 (r0=r9+52)
            cpu.r[0] = lr9.wrapping_add(52);
            cpu.set_t(cpu.fr[2] == cpu.fr[7]); // fcmp/eq fr7,fr2
            if cpu.fr[2] != cpu.fr[7] {
                cpu.fr[0] = cpu.fr[0] + cpu.fr[2]; // delay slot: fadd fr2,fr0
                                                   // bra 0x8c0b9870 -> TAIL2
                lr13 = lr13.wrapping_add(1); // add #1,r13
                cpu.r[0] = 4; // mov #4,r0
                cpu.set_t(4 > lr13); // cmp/gt r13,r0
                lr10 = lr10.wrapping_add(4); // delay slot: add #4,r10
                if 4 > lr13 {
                    continue;
                }
                break;
            }
        }
        // NULLCHK 0x8c0b97ce
        cpu.set_t(lr9 == 0); // tst r9,r9
        if lr9 == 0 {
            // NULL branch 0x8c0b982a
            cpu.r[0] = mem.read_u8(lr14).unwrap() as u32; // mov.b @r14,r0 ; extu.b
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            let mut fr8;
            if cpu.r[0] != 0 {
                let mut r5 = lr13 << 1; // mov r13,r5 ; shll r5
                cpu.set_t(lr13 >> 31 != 0);
                r5 = r5.wrapping_add(lr13); // add r13,r5
                r5 <<= 2; // shll2 r5
                r5 <<= 1; // shll r5 -> 24*r13
                cpu.set_t((r5 >> 1) >> 31 != 0);
                r5 = r5.wrapping_add(lr8); // add r8,r5
                cpu.r[5] = r5;
                fr8 = cpu.fr[4]; // fmov fr4,fr8
                cpu.fr[8] = fr8;
                let mut lr6 = cpu.r[1]; // mov r1,r6
                let mut lr9b = cpu.r[1]; // mov r1,r9
                cpu.r[6] = lr6;
                loop {
                    // INNER2 0x8c0b9846
                    cpu.r[0] = lr9b; // mov r9,r0
                    let e = mem.read_u32(r5.wrapping_add(lr9b)).unwrap(); // mov.l @(r0,r5),r0
                    cpu.r[0] = e;
                    cpu.set_t(e == 0); // tst r0,r0
                    if e != 0 {
                        cpu.r[0] = e.wrapping_add(118); // add #118,r0
                        let w = mem.read_u16(cpu.r[0]).unwrap(); // mov.w @r0,r11 (r11 scratch)
                        let _lr11 = (w as i16 as i32) as u32;
                        cpu.fr[2] = mem.read_f32(0x8c0b9898).unwrap(); // mova ; fmov @r0,fr2 = 10.0
                        cpu.r[0] = ((w as i16 as i32) as u32) & 0xffff; // extu.w r11,r0
                        cpu.fpul = cpu.r[0]; // lds r0,fpul
                        cpu.fr[3] = cpu.fpul as i32 as f32; // float fpul,fr3
                        cpu.fr[3] = cpu.fr[3] / cpu.fr[2]; // fdiv fr2,fr3
                        fr8 = fr8 + cpu.fr[3];
                        cpu.fr[8] = fr8; // fadd fr3,fr8
                    }
                    lr6 = lr6.wrapping_add(1); // add #1,r6
                    cpu.r[6] = lr6;
                    cpu.set_t(5 > lr6); // cmp/gt r6,r3
                    lr9b = lr9b.wrapping_add(4); // delay slot: add #4,r9
                    if 5 > lr6 {
                        continue;
                    }
                    break;
                }
            } else {
                fr8 = cpu.fr[1]; // fmov fr1,fr8 at 0x8c0b986c
            }
            cpu.fr[8] = fr8;
            cpu.fr[0] = cpu.fr[0] + fr8; // fadd fr8,fr0 at 0x8c0b986e
        } else {
            // r9 != 0: 0x8c0b97d2
            cpu.r[0] = mem.read_u8(lr14).unwrap() as u32; // mov.b @r14,r0 ; extu.b
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            cpu.r[5] = mem.read_u32(lr9).unwrap(); // delay slot of bt.s: mov.l @r9,r5
            let mut fr8;
            if cpu.r[0] != 0 {
                let mut r6 = cpu.r[5] << 1; // mov r5,r6 ; shll r6
                cpu.set_t(cpu.r[5] >> 31 != 0);
                r6 = r6.wrapping_add(cpu.r[5]); // add r5,r6
                r6 <<= 2; // shll2 r6
                r6 <<= 1; // shll r6 -> 24 * [r9]
                cpu.set_t((r6 >> 1) >> 31 != 0);
                r6 = r6.wrapping_add(lr8); // add r8,r6
                cpu.r[6] = r6;
                fr8 = cpu.fr[6]; // fmov fr6,fr8
                cpu.fr[8] = fr8;
                let mut lr12 = cpu.r[7]; // mov r7,r12
                let mut lr11 = cpu.r[7]; // mov r7,r11
                loop {
                    // INNER1 0x8c0b97f0
                    cpu.r[0] = lr11; // mov r11,r0
                    let e = mem.read_u32(r6.wrapping_add(lr11)).unwrap(); // mov.l @(r0,r6),r0
                    cpu.r[0] = e;
                    cpu.set_t(e == 0); // tst r0,r0
                    if e != 0 {
                        cpu.r[0] = e.wrapping_add(118); // add #118,r0
                        let w = mem.read_u16(cpu.r[0]).unwrap(); // mov.w @r0,r5
                        cpu.r[5] = (w as i16 as i32) as u32;
                        cpu.fr[2] = mem.read_f32(0x8c0b9898).unwrap(); // mova ; fmov @r0,fr2 = 10.0
                        cpu.r[0] = cpu.r[5] & 0xffff; // extu.w r5,r0
                        cpu.fpul = cpu.r[0]; // lds r0,fpul
                        cpu.fr[3] = cpu.fpul as i32 as f32; // float fpul,fr3
                        cpu.fr[3] = cpu.fr[3] / cpu.fr[2]; // fdiv fr2,fr3
                        fr8 = fr8 + cpu.fr[3];
                        cpu.fr[8] = fr8; // fadd fr3,fr8
                    }
                    lr12 = lr12.wrapping_add(1); // add #1,r12
                    cpu.set_t(5 > lr12); // cmp/gt r12,r3
                    lr11 = lr11.wrapping_add(4); // delay slot: add #4,r11
                    if 5 > lr12 {
                        continue;
                    }
                    break;
                }
            } else {
                fr8 = cpu.fr[5]; // fmov fr5,fr8 at 0x8c0b9816
            }
            cpu.fr[8] = fr8;
            // TAIL 0x8c0b9818
            let addr = lr9.wrapping_add(52); // add #52,r9
            mem.write_f32(addr, fr8).unwrap(); // fmov fr8,@r9
            cpu.r[0] = lr10; // mov r10,r0
            let e2 = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap(); // mov.l @(r0,r4),r0
            cpu.r[0] = e2.wrapping_add(52); // add #52,r0
            cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr2
            cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2
            cpu.fr[0] = cpu.fr[2]; // delay slot of bra: fmov fr2,fr0
        }
        // TAIL2 0x8c0b9870
        lr13 = lr13.wrapping_add(1); // add #1,r13
        cpu.r[0] = 4; // mov #4,r0
        cpu.set_t(4 > lr13); // cmp/gt r13,r0
        lr10 = lr10.wrapping_add(4); // delay slot of bt.s: add #4,r10
        if 4 > lr13 {
            continue;
        }
        break;
    }
    // epilogue pops r8..r14 and r15 unchanged
}
