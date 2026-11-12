//! 0x8c1ac828 - mark every entry of a table with a "seen" flag.
//!
//! Does nothing when the byte at 0x8c2b79e0 is zero. Otherwise `r4+16` selects a
//! mode (0..3, anything else is 0) which is passed to 0x8c074d58 twice, with
//! codes 6 and 7; a zero return from either aborts. Then the table at
//! `[0x8c23dab0]` is walked: for `i < [table+64]`, `r5 = [[table+60] + 4*i]` and
//! the byte at `r5+68` becomes 1 when it was zero, 0 otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u32(0x8c1ac8c4).unwrap_or(0x8c2b79e0); // mov.l 0x8c1ac8c4,r0
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32; // mov.b @r0,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t((cpu.r[0] & cpu.r[0]) == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0); // mov.l @(16,r4),r0
        cpu.r[14] = 0; // mov #0,r14
        cpu.set_t((cpu.r[0] & cpu.r[0]) == 0); // tst r0,r0
        if !cpu.t() {
            cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
            if cpu.t() {
                cpu.r[14] = 1; // bra delay: mov #1,r14
            } else {
                cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
                if cpu.t() {
                    cpu.r[14] = 2; // bra delay: mov #2,r14
                } else {
                    cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
                    if cpu.t() {
                        cpu.r[14] = 3; // mov #3,r14
                    }
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c1ac8c8).unwrap_or(0x8c264da8); // mov.l 0x8c1ac8c8,r0
        cpu.r[5] = cpu.r[14]; // mov r14,r5
        cpu.r[7] = 0; // mov #0,r7
        cpu.r[13] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r13
        cpu.r[6] = 6; // mov #6,r6
        cpu.r[0] = mem.read_u32(0x8c1ac8cc).unwrap_or(0x8c074d58); // mov.l 0x8c1ac8cc,r0
        cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
        cpu.pr = 0x8c1ac870;
        crate::registry::p_8c074d58::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t((cpu.r[0] & cpu.r[0]) == 0); // tst r0,r0
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1ac8cc).unwrap_or(0x8c074d58); // mov.l 0x8c1ac8cc,r0
            cpu.r[5] = cpu.r[14]; // mov r14,r5
            cpu.r[4] = cpu.r[13]; // mov r13,r4
            cpu.r[7] = 0; // mov #0,r7
            cpu.r[6] = 7; // jsr delay: mov #7,r6
            cpu.pr = 0x8c1ac882;
            crate::registry::p_8c074d58::run(cpu, mem);
            cpu.r[0] &= 0xff; // extu.b r0,r0
            cpu.set_t((cpu.r[0] & cpu.r[0]) == 0); // tst r0,r0
            if !cpu.t() {
                cpu.r[0] = mem.read_u32(0x8c1ac8d0).unwrap_or(0x8c23dab0); // mov.l 0x8c1ac8d0,r0
                cpu.r[6] = 0; // mov #0,r6
                cpu.r[13] = cpu.r[6]; // mov r6,r13
                cpu.r[7] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r7
                cpu.r[4] = 68; // mov #68,r4
                cpu.r[1] = cpu.r[7]; // mov r7,r1
                cpu.r[1] = cpu.r[1].wrapping_add(64); // bra delay: add #64,r1
                loop {
                    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r0
                    cpu.set_t((cpu.r[0] as i32) > (cpu.r[6] as i32)); // cmp/gt r6,r0
                    if !cpu.t() {
                        break;
                    }
                    cpu.r[3] = mem.read_u32(cpu.r[7].wrapping_add(60)).unwrap_or(0); // mov.l @(60,r7),r3
                    cpu.r[0] = cpu.r[13]; // mov r13,r0
                    cpu.r[13] = cpu.r[13].wrapping_add(4); // add #4,r13
                    cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0); // mov.l @(r0,r3),r5
                    cpu.r[6] = cpu.r[6].wrapping_add(1); // add #1,r6
                    cpu.r[0] = cpu.r[5]; // mov r5,r0
                    cpu.r[0] = cpu.r[0].wrapping_add(68); // add #68,r0
                    cpu.r[3] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32; // mov.b @r0,r3
                    cpu.r[3] &= 0xff; // extu.b r3,r3
                    cpu.r[3] &= 0xff; // extu.b r3,r3
                    cpu.set_t((cpu.r[3] & cpu.r[3]) == 0); // tst r3,r3
                    cpu.r[0] = cpu.r[4]; // mov r4,r0
                    cpu.r[3] = u32::from(cpu.t()); // movt r3
                    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[3] as u8); // mov.b r3,@(r0,r5)
                }
            }
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
