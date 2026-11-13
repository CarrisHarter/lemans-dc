//! 0x8c23b5ce — first-fit a block of `(i16)r7`'s size class for `r5` bytes.
//!
//! `r5` is rounded up to a multiple of 32 when its low five bits are set.
//! The bin list at `[0x8c23d798 + 36 + index*16 + 8]` is walked for the first
//! node whose size at `+16` is at least the request. An exact node is unlinked
//! (`0x8c23b9d0`) and reinserted (`0x8c23b990`). A larger node is split into a
//! slot from `0x8c23b8c4`; no node, or no free slot, returns 3. Otherwise
//! `*r6` receives the payload pointer at `+12`, the caller's `r4` is merged
//! into the node's flag halfword, and `r0` is 0. `r8`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn read_w(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn write_w(mem: &mut Memory, addr: u32, value: u32) {
    let _ = mem.write_u16(addr, value as u16);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c23b5ce: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c23b5d0: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c23b5d2: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c23b5d4: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c23b5d6: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    // 0x8c23b5d8: mov.l r9,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    // 0x8c23b5da: mov.l r8,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    // 0x8c23b5dc: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c23b5de: add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // 0x8c23b5e0: mov #31,r12
    cpu.r[12] = 31;
    // 0x8c23b5e2: mov.l r4,@(4,r15)
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[4]);
    // 0x8c23b5e4: tst r5,r12
    cpu.set_t((cpu.r[5] & cpu.r[12]) == 0);
    // 0x8c23b5e6: mov r6,r11
    cpu.r[11] = cpu.r[6];
    // 0x8c23b5e8: bf.s 0x8c23b5f0 ; delay 0x8c23b5ea always
    cpu.r[8] = cpu.r[7];
    if cpu.t() {
        // 0x8c23b5ec: bra 0x8c23b5f8 ; delay 0x8c23b5ee mov r5,r12
        cpu.r[12] = cpu.r[5];
    } else {
        // 0x8c23b5f0: mov r5,r12
        cpu.r[12] = cpu.r[5];
        // 0x8c23b5f2: mov #-32,r3
        cpu.r[3] = (-32i32) as u32;
        // 0x8c23b5f4: add #32,r12
        cpu.r[12] = cpu.r[12].wrapping_add(32);
        // 0x8c23b5f6: and r3,r12
        cpu.r[12] &= cpu.r[3];
    }
    // 0x8c23b5f8: mov.l 0x8c23b620,r10
    cpu.r[10] = mem.read_u32(0x8c23b620).unwrap_or(0);
    // 0x8c23b5fa: exts.w r8,r9
    cpu.r[9] = cpu.r[8] as i16 as i32 as u32;
    // 0x8c23b5fc: shll2 r9
    cpu.r[9] <<= 2;
    // 0x8c23b5fe: mov r10,r4
    cpu.r[4] = cpu.r[10];
    // 0x8c23b600: add #36,r4
    cpu.r[4] = cpu.r[4].wrapping_add(36);
    // 0x8c23b602: shll2 r9
    cpu.r[9] <<= 2;
    // 0x8c23b604: add r9,r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[9]);
    // 0x8c23b606: mov.l @(8,r4),r4
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c23b608: mov #0,r5
    cpu.r[5] = 0;
    // 0x8c23b60a: tst r4,r4
    cpu.set_t(cpu.r[4] == 0);
    // 0x8c23b60c: bt.s 0x8c23b646 ; delay 0x8c23b60e always
    cpu.r[14] = cpu.r[5];
    if cpu.r[4] != 0 {
        loop {
            // 0x8c23b610: mov.l @(16,r4),r2
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
            // 0x8c23b612: cmp/hs r12,r2
            cpu.set_t(cpu.r[2] >= cpu.r[12]);
            // 0x8c23b614: bf 0x8c23b640
            if !cpu.t() {
                // 0x8c23b640: mov.l @(8,r4),r4
                cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            } else {
                // 0x8c23b616: mov r4,r14
                cpu.r[14] = cpu.r[4];
                // 0x8c23b618: bra 0x8c23b642 ; delay 0x8c23b61a mov r5,r4
                cpu.r[4] = cpu.r[5];
            }
            // 0x8c23b642: tst r4,r4
            cpu.set_t(cpu.r[4] == 0);
            // 0x8c23b644: bf 0x8c23b610
            if cpu.r[4] == 0 {
                break;
            }
        }
    }
    // 0x8c23b646: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c23b648: bt 0x8c23b6a2
    if cpu.r[14] == 0 {
        // 0x8c23b6a2: bra 0x8c23b6ea ; delay mov #3,r0
        cpu.r[0] = 3;
    } else {
        // 0x8c23b64a: mov.l @(16,r14),r3
        cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0);
        // 0x8c23b64c: cmp/eq r12,r3
        cpu.set_t(cpu.r[12] == cpu.r[3]);
        // 0x8c23b64e: bf 0x8c23b698
        if cpu.t() {
            // 0x8c23b650: mov r10,r5
            cpu.r[5] = cpu.r[10];
            // 0x8c23b652: add #36,r5
            cpu.r[5] = cpu.r[5].wrapping_add(36);
            // 0x8c23b654: add r9,r5
            cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[9]);
            // 0x8c23b656: mov r14,r6
            cpu.r[6] = cpu.r[14];
            // 0x8c23b658: mov.l r5,@r15
            let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
            // 0x8c23b65a: add #12,r5
            cpu.r[5] = cpu.r[5].wrapping_add(12);
            // 0x8c23b65c: mov.l @r15,r4
            cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            // 0x8c23b65e: bsr 0x8c23b9d0 ; delay 0x8c23b660 add #8,r4
            cpu.r[4] = cpu.r[4].wrapping_add(8);
            cpu.pr = 0x8c23b662;
            crate::registry::p_8c23b9d0::run(cpu, mem);
            // 0x8c23b662: mov r10,r5
            cpu.r[5] = cpu.r[10];
            // 0x8c23b664: add #36,r5
            cpu.r[5] = cpu.r[5].wrapping_add(36);
            // 0x8c23b666: add r9,r5
            cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[9]);
            // 0x8c23b668: mov r14,r6
            cpu.r[6] = cpu.r[14];
            // 0x8c23b66a: mov.l r5,@r15
            let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
            // 0x8c23b66c: add #4,r5
            cpu.r[5] = cpu.r[5].wrapping_add(4);
            // 0x8c23b66e: bsr 0x8c23b990 ; delay 0x8c23b670 mov.l @r15,r4
            cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.pr = 0x8c23b672;
            crate::registry::p_8c23b990::run(cpu, mem);
            // 0x8c23b672: mov.l @(12,r14),r2
            cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
            // 0x8c23b674: exts.w r8,r8
            cpu.r[8] = cpu.r[8] as i16 as i32 as u32;
            // 0x8c23b676: tst r8,r8
            cpu.set_t(cpu.r[8] == 0);
            // 0x8c23b678: mov #1,r3
            cpu.r[3] = 1;
            // 0x8c23b67a: mov.l r2,@r11
            let _ = mem.write_u32(cpu.r[11], cpu.r[2]);
            // 0x8c23b67c: mov.l r11,@(20,r14)
            let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[11]);
            // 0x8c23b67e: bf.s 0x8c23b688 ; delay 0x8c23b680 always
            write_w(mem, cpu.r[14], cpu.r[3]);
            if cpu.t() {
                // 0x8c23b682: mov.w @r14,r0
                cpu.r[0] = read_w(mem, cpu.r[14]);
                // 0x8c23b684: bra ; delay 0x8c23b686 or #16,r0
                cpu.r[0] |= 16;
            } else {
                // 0x8c23b688: mov.w @r14,r0
                cpu.r[0] = read_w(mem, cpu.r[14]);
                // 0x8c23b68a: or #32,r0
                cpu.r[0] |= 32;
            }
            // 0x8c23b68c: mov.w r0,@r14
            write_w(mem, cpu.r[14], cpu.r[0]);
            // 0x8c23b68e: mov.w @r14,r2
            cpu.r[2] = read_w(mem, cpu.r[14]);
            // 0x8c23b690: mov.l @(4,r15),r3
            cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
            // 0x8c23b692: or r3,r2
            cpu.r[2] |= cpu.r[3];
            // 0x8c23b694: bra 0x8c23b6e8 ; delay 0x8c23b696 mov.w r2,@r14
            write_w(mem, cpu.r[14], cpu.r[2]);
            // 0x8c23b6e8: mov #0,r0
            cpu.r[0] = 0;
        } else {
            // 0x8c23b698: bsr 0x8c23b8c4 ; delay nop. Return 0x8c23b69c.
            cpu.pr = 0x8c23b69c;
            crate::registry::p_8c23b8c4::run(cpu, mem);
            // 0x8c23b69c: mov r0,r13
            cpu.r[13] = cpu.r[0];
            // 0x8c23b69e: tst r13,r13
            cpu.set_t(cpu.r[13] == 0);
            // 0x8c23b6a0: bf 0x8c23b6a6
            if cpu.r[13] == 0 {
                // 0x8c23b6a2: bra 0x8c23b6ea ; delay mov #3,r0
                cpu.r[0] = 3;
            } else {
                // 0x8c23b6a6: mov r10,r5
                cpu.r[5] = cpu.r[10];
                // 0x8c23b6a8: add #36,r5
                cpu.r[5] = cpu.r[5].wrapping_add(36);
                // 0x8c23b6aa: mov.l @(12,r14),r3
                cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
                // 0x8c23b6ac: add r9,r5
                cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[9]);
                // 0x8c23b6ae: mov r13,r6
                cpu.r[6] = cpu.r[13];
                // 0x8c23b6b0: mov.l r3,@(12,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[3]);
                // 0x8c23b6b2: mov.l r12,@(16,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[12]);
                // 0x8c23b6b4: mov.l @(16,r14),r4
                cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0);
                // 0x8c23b6b6: sub r12,r4
                cpu.r[4] = cpu.r[4].wrapping_sub(cpu.r[12]);
                // 0x8c23b6b8: mov.l r4,@(16,r14)
                let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[4]);
                // 0x8c23b6ba: mov.l @(12,r14),r2
                cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
                // 0x8c23b6bc: add r12,r2
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[12]);
                // 0x8c23b6be: mov.l r2,@(12,r14)
                let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[2]);
                // 0x8c23b6c0: mov.l r5,@r15
                let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
                // 0x8c23b6c2: add #4,r5
                cpu.r[5] = cpu.r[5].wrapping_add(4);
                // 0x8c23b6c4: bsr 0x8c23b990 ; delay 0x8c23b6c6 mov.l @r15,r4
                cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.pr = 0x8c23b6c8;
                crate::registry::p_8c23b990::run(cpu, mem);
                // 0x8c23b6c8: mov.l @(12,r13),r2
                cpu.r[2] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
                // 0x8c23b6ca: exts.w r8,r8
                cpu.r[8] = cpu.r[8] as i16 as i32 as u32;
                // 0x8c23b6cc: tst r8,r8
                cpu.set_t(cpu.r[8] == 0);
                // 0x8c23b6ce: mov.l r2,@r11
                let _ = mem.write_u32(cpu.r[11], cpu.r[2]);
                // 0x8c23b6d0: bf.s 0x8c23b6da ; delay 0x8c23b6d2 always
                let _ = mem.write_u32(cpu.r[13].wrapping_add(20), cpu.r[11]);
                if cpu.t() {
                    // 0x8c23b6d4: mov.w @r13,r0
                    cpu.r[0] = read_w(mem, cpu.r[13]);
                    // 0x8c23b6d6: bra ; delay or #16,r0
                    cpu.r[0] |= 16;
                } else {
                    // 0x8c23b6da: mov.w @r13,r0
                    cpu.r[0] = read_w(mem, cpu.r[13]);
                    // 0x8c23b6dc: or #32,r0
                    cpu.r[0] |= 32;
                }
                // 0x8c23b6de: mov.w r0,@r13
                write_w(mem, cpu.r[13], cpu.r[0]);
                // 0x8c23b6e0: mov.w @r13,r2
                cpu.r[2] = read_w(mem, cpu.r[13]);
                // 0x8c23b6e2: mov.l @(4,r15),r3
                cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
                // 0x8c23b6e4: or r3,r2
                cpu.r[2] |= cpu.r[3];
                // 0x8c23b6e6: mov.w r2,@r13
                write_w(mem, cpu.r[13], cpu.r[2]);
                // 0x8c23b6e8: mov #0,r0
                cpu.r[0] = 0;
            }
        }
    }
    // 0x8c23b6ea: add #8,r15
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    // 0x8c23b6ec: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6ee: mov.l @r15+,r8
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6f0: mov.l @r15+,r9
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6f2: mov.l @r15+,r10
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6f4: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6f6: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6f8: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23b6fa: rts ; delay 0x8c23b6fc mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
