//! 0x8c216008 — count bit positions until the first set bit: scans `r5*64`
//! bytes at `r4`, testing bit 7 down to bit 0 of each against mask r0=0x80;
//! returns the number of shifts performed in `r0`.
//!
//! Original:
//! ```text
//! mov.l r14,@-r15 ; r14 = r5 ; shll2 r14 ; mov.l r12,@-r15 ; r12 = 8
//! shll2 r14 ; r0 = r12 ; r7 = 0 ; shll2 r14 ; bra check ; add #120,r0 (delay)
//! ; r0 = 128, r14 = r5*64
//! body: mov.b @r4+,r6 ; mov r12,r5          (r5 = 8 per byte)
//! inner: exts.b r6,r3 ; tst r0,r3 ; bf done (bit set -> return r7)
//!        add #-1,r5 ; shll r6 ; cmp/pl r5 ; bt.s inner ; add #1,r7 (delay)
//!        add #-1,r14
//! check: cmp/pl r14 ; bt body
//! done: mov.l @r15+,r12 ; mov r7,r0 ; rts ; mov.l @r15+,r14 (delay)
//! ```
//! The `bt.s` delay slot `add #1,r7` runs on the final non-taken pass too, so
//! every byte contributes 8 to r7 unless a set bit exits early. r12/r14 are
//! restored in the epilogue; T ends 0 on both exits.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r14_orig = cpu.r[14];
    let r12_orig = cpu.r[12];
    let mut r14 = cpu.r[5] << 2; // shll2
    r14 <<= 2; // shll2
    r14 <<= 2; // shll2: r14 = r5 * 64
    cpu.r[12] = 8;
    let mut r0 = 8u32.wrapping_add(120); // mov r12,r0 ; bra delay add #120,r0
    cpu.r[0] = r0;
    let mut r7 = 0u32;
    cpu.r[7] = r7;
    let mut r4 = cpu.r[4];
    // bra 0x8c216034: the check runs before the first body
    loop {
        cpu.set_t((r14 as i32) > 0); // cmp/pl r14
        if (r14 as i32) <= 0 {
            break;
        }
        // 0x8c21601e
        let mut r6 = mem.read_u8(r4).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4+,r6
        r4 = r4.wrapping_add(1);
        cpu.r[4] = r4;
        cpu.r[6] = r6;
        let mut r5 = 8u32; // mov r12,r5
        cpu.r[5] = r5;
        loop {
            // 0x8c216022 inner head
            let r3 = r6 as u8 as i8 as i32 as u32; // exts.b r6,r3
            cpu.r[3] = r3;
            cpu.set_t((r0 & r3) == 0); // tst r0,r3
            if (r0 & r3) != 0 {
                // bf 0x8c216038: early exit with r0 = r7
                cpu.r[5] = r5;
                cpu.r[7] = r7;
                cpu.r[0] = r7;
                cpu.r[12] = r12_orig;
                cpu.r[14] = r14_orig;
                return;
            }
            r5 = r5.wrapping_sub(1);
            cpu.r[5] = r5;
            cpu.set_t(r6 >> 31 != 0); // shll r6
            r6 = r6 << 1;
            cpu.r[6] = r6;
            cpu.set_t((r5 as i32) > 0); // cmp/pl r5
            r7 = r7.wrapping_add(1); // bt.s delay slot runs taken or not
            cpu.r[7] = r7;
            if (r5 as i32) > 0 {
                continue;
            }
            break;
        }
        r14 = r14.wrapping_sub(1); // add #-1,r14
    }
    cpu.r[4] = r4;
    cpu.r[0] = r7; // mov r7,r0
    cpu.r[12] = r12_orig; // mov.l @r15+,r12
    cpu.r[14] = r14_orig; // rts delay: mov.l @r15+,r14
}
