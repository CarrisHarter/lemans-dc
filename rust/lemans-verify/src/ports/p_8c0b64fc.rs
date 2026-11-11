//! 0x8c0b64fc — scan a byte range for a pattern; returns an offset or -1.
//!
//! ```text
//! push r14/r13/r12 (stack window; net r15 unchanged, regs restored)
//! mov.l @r4,r0 ; mov.l @r0,r4     ; r4 = count = **r4  (r4 clobbered!)
//! cmp/hs r4,r6 ; bt 0x8c0b650e    ; FAIL if r6 >= count (unsigned)
//! tst r7,r7    ; bf 0x8c0b6512    ; FAIL if r7 == 0
//! 0x8c0b650e: bra 0x8c0b6548 ; mov #-1,r0 (delay)
//! 0x8c0b6512: mov.l @(12,r0),r1 ; mov #0,r14 ; r0 = r1 ; r3 = r1
//!             add r6,r0 ; bra 0x8c0b6542 ; delay: add r4,r3
//! outer 0x8c0b6520: mov r5,r13 ; cmp/hi r14,r7 ; bf.s 0x8c0b653c ; delay: mov r14,r12
//! inner 0x8c0b6528: mov.b @r0,r6 ; mov.b @r13,r4 ; exts.b r6,r6 ; exts.b r4,r4
//!                   cmp/eq r4,r6 ; bt 0x8c0b6540
//!                   add #1,r12 ; cmp/hi r12,r7 ; bt.s 0x8c0b6528 ; delay: add #1,r13
//! 0x8c0b653c: bra 0x8c0b6548 ; delay: sub r1,r0   (return r0 - base)
//! 0x8c0b6540: add #1,r0
//! 0x8c0b6542: cmp/eq r0,r3 ; bf 0x8c0b6520 ; mov #-1,r0
//! epilogue: pop r12/r13 ; rts ; delay: pop r14
//! ```
//! Hidden side effects: r4 ends as the last sign-extended pattern byte (the
//! argument is destroyed), r6 as the last text byte, r1/r3 as base and
//! base+count, r12/r13/r14 restored from the stack, and T is 0 on the
//! offset-return paths but 1 on the -1-via-end path (or from the entry
//! compare when FAILing).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let (s12, s13, s14) = (cpu.r[12], cpu.r[13], cpu.r[14]);
    let mut r0 = mem.read_u32(cpu.r[4]).unwrap();
    let r4 = mem.read_u32(r0).unwrap(); // count — clobbers the r4 argument
    let r6seed = cpu.r[6];
    let r7 = cpu.r[7];
    let r5 = cpu.r[5];

    // cmp/hs r4,r6 → T = r6 >= r4 (unsigned); bt 0x8c0b650e → FAIL
    if r6seed >= r4 {
        cpu.set_t(true);
        cpu.r[0] = 0xFFFF_FFFF; // bra delay: mov #-1,r0
        cpu.r[4] = r4;
    } else {
        cpu.set_t(r7 == 0); // tst r7,r7
        if r7 == 0 {
            // bf not taken → fall through into 0x8c0b650e → FAIL
            cpu.r[0] = 0xFFFF_FFFF;
            cpu.r[4] = r4;
        } else {
            let r1 = mem.read_u32(r0.wrapping_add(12)).unwrap(); // base
            let r14: u32 = 0; // mov #0,r14 — never written again
            let mut r3 = r1;
            r0 = r1.wrapping_add(r6seed); // start = base + r6
            r3 = r3.wrapping_add(r4); // end = base + count (delay slot)
            let mut r6l = r6seed;
            let mut r4l = r4;
            'outer: loop {
                // 0x8c0b6520
                let mut r13 = r5; // mov r5,r13
                // cmp/hi r14,r7 → T = r7 > r14 ; bf.s 0x8c0b653c
                if !(r7 > r14) {
                    r0 = r0.wrapping_sub(r1); // 0x8c0b653c delay: sub r1,r0
                    cpu.set_t(false);
                    break;
                }
                let mut r12 = r14; // delay slot: mov r14,r12
                loop {
                    // 0x8c0b6528
                    r6l = mem.read_u8(r0).unwrap() as u8 as i8 as i32 as u32;
                    r4l = mem.read_u8(r13).unwrap() as u8 as i8 as i32 as u32;
                    r6l = r6l as u8 as i8 as i32 as u32; // exts.b r6,r6
                    r4l = r4l as u8 as i8 as i32 as u32; // exts.b r4,r4
                    // cmp/eq r4,r6 ; bt 0x8c0b6540
                    if r4l == r6l {
                        r0 = r0.wrapping_add(1); // 0x8c0b6540: add #1,r0
                        // 0x8c0b6542: cmp/eq r0,r3 ; bf 0x8c0b6520
                        if r0 == r3 {
                            cpu.set_t(true);
                            r0 = 0xFFFF_FFFF; // 0x8c0b6546: mov #-1,r0
                            break 'outer;
                        }
                        cpu.set_t(false);
                        continue 'outer;
                    }
                    r12 = r12.wrapping_add(1); // add #1,r12
                    // cmp/hi r12,r7 → T = r7 > r12 ; bt.s 0x8c0b6528
                    if r7 > r12 {
                        r13 = r13.wrapping_add(1); // delay: add #1,r13
                        continue;
                    }
                    // fall through to 0x8c0b653c
                    r0 = r0.wrapping_sub(r1); // bra delay: sub r1,r0
                    cpu.set_t(false);
                    break 'outer;
                }
            }
            cpu.r[0] = r0;
            cpu.r[1] = r1;
            cpu.r[3] = r3;
            cpu.r[4] = r4l;
            cpu.r[6] = r6l;
        }
    }
    cpu.r[12] = s12; // pop r12
    cpu.r[13] = s13; // pop r13
    cpu.r[14] = s14; // rts delay: pop r14
}
