//! 0x8c144784 — reverse scan of `(*r4)`'s buffer for any byte of an `r7`-byte
//! character set at `r5`.
//!
//! `*r4` is a descriptor: `[0]` is a count and `[12]` is the buffer base
//! (0x8c01ec2c is `return *(u32 *)(p+12);` and is called with `r4 = *r4`).
//! The start index is `min(r6, count-1)`; the scan walks down from
//! `base+start` to `base`, and each position is compared against every byte of
//! the set. A hit returns the byte's index (`r14 - base - 1`); a full miss
//! returns -1. The comparison at 0x7c4 therefore returns -1 straight away.
//!
//! The prologue spills r14/r13/r12/r11/r10/pr and the epilogue restores them,
//! so every one of those registers ends at its entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c144784: mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[7]; // 8c14478a: mov r7,r13
    // 8c14478c: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[6]; // 8c14478e: mov r6,r12
    // 8c144790: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[11] = cpu.r[5]; // 8c144792: mov r5,r11
    // 8c144794: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[10] = cpu.r[4]; // 8c144796: mov r4,r10
    cpu.r[4] = mem.read_u32(cpu.r[10]).unwrap_or(0); // 8c144798: mov.l @r10,r4
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // 8c14479a: mov.l @r4,r1
    cpu.set_t(cpu.r[1] == 0); // 8c14479c: tst r1,r1 ; bt 0x8c1447a4
    let empty = cpu.t();
    if !empty {
        cpu.set_t(cpu.r[13] == 0); // 8c1447a0: tst r13,r13 ; bf 0x8c1447a8
    }
    if empty || cpu.t() {
        // 8c1447a4: bra 0x8c144802 ; mov #-1,r0 (delay slot)
        cpu.r[0] = 0xffff_ffff;
    } else {
        cpu.r[0] = 0xffff_ffff; // 8c1447a8: mov #-1,r0
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // 8c1447aa: add r0,r1
        cpu.set_t(cpu.r[12] > cpu.r[1]); // 8c1447ac: cmp/hi r1,r12
        if cpu.t() {
            cpu.r[12] = cpu.r[1]; // 8c1447b0: mov r1,r12 — clamp start to count-1
        }
        // 8c1447b2: mov.l @(0x8c144810,pc),r0 — literal 0x8c01ec2c
        cpu.r[0] = mem.read_u32(0x8c144810).unwrap_or(0);
        cpu.pr = 0x8c1447b8; // 8c1447b4: jsr @r0 ; nop
        crate::registry::p_8c01ec2c::run(cpu, mem);

        cpu.r[14] = cpu.r[0]; // 8c1447b8: mov r0,r14
        cpu.r[4] = mem.read_u32(cpu.r[10]).unwrap_or(0); // 8c1447ba: mov.l @r10,r4
        cpu.r[12] = cpu.r[12].wrapping_add(1); // 8c1447bc: add #1,r12
        cpu.r[0] = mem.read_u32(0x8c144810).unwrap_or(0); // 8c1447be: literal reload
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[12]); // 8c1447c2: delay slot add r12,r14
        cpu.pr = 0x8c1447c4; // 8c1447c0: jsr @r0
        crate::registry::p_8c01ec2c::run(cpu, mem);
        cpu.set_t(cpu.r[14] == cpu.r[0]); // 8c1447c4: cmp/eq r0,r14 ; bt 0x8c144800
        if cpu.t() {
            cpu.r[0] = 0xffff_ffff; // 8c144800: mov #-1,r0
        } else {
            loop {
                cpu.r[5] = 0; // 8c1447c8: mov #0,r5
                cpu.set_t(cpu.r[13] > cpu.r[5]); // 8c1447ca: cmp/hi r5,r13 ; bf.s 0x8c1447f4
                let mut hit = false;
                if cpu.t() {
                    cpu.r[4] = cpu.r[11]; // 8c1447ce: mov r11,r4 (delay slot, always runs)
                    loop {
                        cpu.r[3] = cpu.r[14]; // 8c1447d0: mov r14,r3
                        cpu.r[3] = cpu.r[3].wrapping_sub(1); // 8c1447d2: add #-1,r3
                        cpu.r[1] = (mem.read_u8(cpu.r[3]).unwrap_or(0) as i8 as i32) as u32; // 8c1447d4
                        cpu.r[0] = (mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32) as u32; // 8c1447d6
                        cpu.r[1] = (cpu.r[1] as u8 as i8 as i32) as u32; // 8c1447d8: exts.b r1,r1
                        cpu.r[0] = (cpu.r[0] as u8 as i8 as i32) as u32; // 8c1447da: exts.b r0,r0
                        cpu.set_t(cpu.r[1] == cpu.r[0]); // 8c1447dc: cmp/eq r0,r1 ; bf 0x8c1447ec
                        if cpu.t() {
                            cpu.r[0] = mem.read_u32(cpu.r[10]).unwrap_or(0); // 8c1447e0
                            cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0); // 8c1447e2
                            cpu.r[14] = cpu.r[14].wrapping_sub(cpu.r[0]); // 8c1447e4: sub r0,r14
                            cpu.r[14] = cpu.r[14].wrapping_sub(1); // 8c1447e6: add #-1,r14
                            cpu.r[0] = cpu.r[14]; // 8c1447ea: bra 0x8c144802 ; mov r14,r0
                            hit = true;
                            break;
                        }
                        cpu.r[5] = cpu.r[5].wrapping_add(1); // 8c1447ec: add #1,r5
                        cpu.set_t(cpu.r[13] > cpu.r[5]); // 8c1447ee: cmp/hi r5,r13
                        cpu.r[4] = cpu.r[4].wrapping_add(1); // 8c1447f2: bt.s 0x8c1447d4 ; add #1,r4
                        if !cpu.t() {
                            break;
                        }
                    }
                } else {
                    cpu.r[4] = cpu.r[11]; // 8c1447ce delay slot when the branch is not taken
                }
                if hit {
                    break;
                }
                cpu.r[4] = mem.read_u32(cpu.r[10]).unwrap_or(0); // 8c1447f4: mov.l @r10,r4
                cpu.r[0] = mem.read_u32(0x8c144810).unwrap_or(0); // 8c1447f6: literal reload
                cpu.r[14] = cpu.r[14].wrapping_sub(1); // 8c1447fa: jsr delay slot add #-1,r14
                cpu.pr = 0x8c1447fc; // 8c1447f8: jsr @r0
                crate::registry::p_8c01ec2c::run(cpu, mem);
                cpu.set_t(cpu.r[14] == cpu.r[0]); // 8c1447fc: cmp/eq r0,r14 ; bf 0x8c1447c8
                if cpu.t() {
                    cpu.r[0] = 0xffff_ffff; // 8c144800: mov #-1,r0
                    break;
                }
            }
        }
    }

    // 8c144802: lds.l @r15+,pr ; mov.l @r15+,r10 ; mov.l @r15+,r11
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c144808: mov.l @r15+,r12 ; mov.l @r15+,r13
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c14480c: rts ; mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
