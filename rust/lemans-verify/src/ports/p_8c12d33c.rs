//! 0x8c12d33c — install a vtable through `[r4]` and, when `(i16)r5 != 0`,
//! finish the second table, tear down the two embedded objects, and free
//! `r4` itself if that halfword is positive.
//!
//! `r4 == 0` returns immediately. Otherwise the low half of `r5` selects:
//!
//! - `0`: pointer wiring only. The `bt.s` delay store still runs.
//! - `< 0`: wiring, the second table, `0x8c0cd048(r4+76, 0)`,
//!   `0x8c07c100(r4+36, 0)`, then `[r4+8] = 0x8c1e9ad4`. No final free.
//! - `> 0`: the same, then `free(r4)` through `0x8c01032c`.
//!
//! `r11`–`r14` are saved and restored. `r0` is the entry `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c12d33c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c12d33e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c12d340: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c12d342: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c12d344: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c12d346: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c12d348: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c12d34a: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c12d34c: bt 0x8c12d430
    if cpu.r[13] != 0 {
        // 0x8c12d34e: mov.l 0x8c12d440,r0
        cpu.r[0] = mem.read_u32(0x8c12d440).unwrap_or(0);
        // 0x8c12d350: mov r13,r7
        cpu.r[7] = cpu.r[13];
        // 0x8c12d352: mov r13,r6
        cpu.r[6] = cpu.r[13];
        // 0x8c12d354: mov.l r0,@(4,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[0]);
        // 0x8c12d356: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c12d358: mov.l 0x8c12d444,r4
        cpu.r[4] = mem.read_u32(0x8c12d444).unwrap_or(0);
        // 0x8c12d35a: mov r0,r3
        cpu.r[3] = cpu.r[0];
        // 0x8c12d35c: mov.l @r13,r5
        cpu.r[5] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d35e: add r4,r3
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        // 0x8c12d360: add #36,r1
        cpu.r[1] = cpu.r[1].wrapping_add(36);
        // 0x8c12d362: mov.l r3,@(8,r5)
        let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
        // 0x8c12d364: mov r0,r12
        cpu.r[12] = cpu.r[0];
        // 0x8c12d366: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d368: mov r13,r5
        cpu.r[5] = cpu.r[13];
        // 0x8c12d36a: add #8,r7
        cpu.r[7] = cpu.r[7].wrapping_add(8);
        // 0x8c12d36c: mov.l @r3,r4
        cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c12d36e: add #36,r6
        cpu.r[6] = cpu.r[6].wrapping_add(36);
        // 0x8c12d370: add #76,r5
        cpu.r[5] = cpu.r[5].wrapping_add(76);
        // 0x8c12d372: mov.l r1,@r4
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
        // 0x8c12d374: mov r13,r3
        cpu.r[3] = cpu.r[13];
        // 0x8c12d376: mov.l 0x8c12d448,r4
        cpu.r[4] = mem.read_u32(0x8c12d448).unwrap_or(0);
        // 0x8c12d378: exts.w r14,r1
        cpu.r[1] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c12d37a: mov.l @r13,r11
        cpu.r[11] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d37c: add r4,r12
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[4]);
        // 0x8c12d37e: mov.l 0x8c12d44c,r4
        cpu.r[4] = mem.read_u32(0x8c12d44c).unwrap_or(0);
        // 0x8c12d380: add #96,r3
        cpu.r[3] = cpu.r[3].wrapping_add(96);
        // 0x8c12d382: mov.l @(4,r11),r11
        cpu.r[11] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
        // 0x8c12d384: add r4,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]);
        // 0x8c12d386: tst r1,r1
        cpu.set_t(cpu.r[1] == 0);
        // 0x8c12d388: mov.l r12,@(4,r11)
        let _ = mem.write_u32(cpu.r[11].wrapping_add(4), cpu.r[12]);
        // 0x8c12d38a: mov.l @r13,r4
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d38c: mov.l @(4,r4),r12
        cpu.r[12] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        // 0x8c12d38e: mov.l @r12,r12
        cpu.r[12] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        // 0x8c12d390: mov r3,r4
        cpu.r[4] = cpu.r[3];
        // 0x8c12d392: mov.l r0,@(4,r12)
        let _ = mem.write_u32(cpu.r[12].wrapping_add(4), cpu.r[0]);
        // 0x8c12d394: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d396: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c12d398: sub r0,r7
        cpu.r[7] = cpu.r[7].wrapping_sub(cpu.r[0]);
        // 0x8c12d39a: mov.l r7,@(24,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[7]);
        // 0x8c12d39c: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d39e: mov.l @(4,r0),r0
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
        // 0x8c12d3a0: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c12d3a2: sub r0,r6
        cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[0]);
        // 0x8c12d3a4: mov.l r6,@(36,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(36), cpu.r[6]);
        // 0x8c12d3a6: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d3a8: mov.l @(4,r0),r0
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
        // 0x8c12d3aa: sub r0,r5
        cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[0]);
        // 0x8c12d3ac: mov.l r5,@(16,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[5]);
        // 0x8c12d3ae: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c12d3b0: sub r0,r4
        cpu.r[4] = cpu.r[4].wrapping_sub(cpu.r[0]);
        // 0x8c12d3b2: bt.s 0x8c12d424 ; delay 0x8c12d3b4 always
        let _ = mem.write_u32(cpu.r[0].wrapping_add(12), cpu.r[4]);
        if !cpu.t() {
            // 0x8c12d3b6: tst r3,r3
            cpu.set_t(cpu.r[3] == 0);
            // 0x8c12d3b8: bt 0x8c12d404
            if cpu.r[3] != 0 {
                // 0x8c12d3ba: mov.l 0x8c12d450,r1
                cpu.r[1] = mem.read_u32(0x8c12d450).unwrap_or(0);
                // 0x8c12d3bc: mov #104,r0
                cpu.r[0] = 104;
                // 0x8c12d3be: mov.l r1,@(r0,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(cpu.r[0]), cpu.r[1]);
                // 0x8c12d3c0: mov r1,r4
                cpu.r[4] = cpu.r[1];
                // 0x8c12d3c2: mov.l @r3,r5
                cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
                // 0x8c12d3c4: add #56,r4
                cpu.r[4] = cpu.r[4].wrapping_add(56);
                // 0x8c12d3c6: mov r13,r0
                cpu.r[0] = cpu.r[13];
                // 0x8c12d3c8: mov.l r4,@r5
                let _ = mem.write_u32(cpu.r[5], cpu.r[4]);
                // 0x8c12d3ca: add #100,r0
                cpu.r[0] = cpu.r[0].wrapping_add(100);
                // 0x8c12d3cc: mov.l 0x8c12d454,r5
                cpu.r[5] = mem.read_u32(0x8c12d454).unwrap_or(0);
                // 0x8c12d3ce: mov r1,r4
                cpu.r[4] = cpu.r[1];
                // 0x8c12d3d0: mov.l @r0,r6
                cpu.r[6] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c12d3d2: add r5,r4
                cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
                // 0x8c12d3d4: mov r13,r5
                cpu.r[5] = cpu.r[13];
                // 0x8c12d3d6: mov.l r4,@(4,r6)
                let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[4]);
                // 0x8c12d3d8: add #108,r5
                cpu.r[5] = cpu.r[5].wrapping_add(108);
                // 0x8c12d3da: mov.l @r0,r6
                cpu.r[6] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c12d3dc: mov.l 0x8c12d458,r4
                cpu.r[4] = mem.read_u32(0x8c12d458).unwrap_or(0);
                // 0x8c12d3de: mov.l @r6,r6
                cpu.r[6] = mem.read_u32(cpu.r[6]).unwrap_or(0);
                // 0x8c12d3e0: add r4,r1
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[4]);
                // 0x8c12d3e2: mov r13,r4
                cpu.r[4] = cpu.r[13];
                // 0x8c12d3e4: mov.l r1,@(4,r6)
                let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[1]);
                // 0x8c12d3e6: mov.l @r3,r3
                cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0);
                // 0x8c12d3e8: mov.l 0x8c12d45c,r1
                cpu.r[1] = mem.read_u32(0x8c12d45c).unwrap_or(0);
                // 0x8c12d3ea: sub r3,r5
                cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[3]);
                // 0x8c12d3ec: mov.l r5,@(24,r3)
                let _ = mem.write_u32(cpu.r[3].wrapping_add(24), cpu.r[5]);
                // 0x8c12d3ee: add r1,r4
                cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
                // 0x8c12d3f0: mov.l 0x8c12d460,r3
                cpu.r[3] = mem.read_u32(0x8c12d460).unwrap_or(0);
                // 0x8c12d3f2: mov r13,r1
                cpu.r[1] = cpu.r[13];
                // 0x8c12d3f4: mov.l @r0,r5
                cpu.r[5] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c12d3f6: add r3,r1
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
                // 0x8c12d3f8: mov.l @r5,r3
                cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
                // 0x8c12d3fa: sub r3,r4
                cpu.r[4] = cpu.r[4].wrapping_sub(cpu.r[3]);
                // 0x8c12d3fc: mov.l r4,@(36,r3)
                let _ = mem.write_u32(cpu.r[3].wrapping_add(36), cpu.r[4]);
                // 0x8c12d3fe: mov.l @r0,r0
                cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c12d400: sub r0,r1
                cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]);
                // 0x8c12d402: mov.l r1,@(16,r0)
                let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[1]);
            }
            // 0x8c12d404: mov.l 0x8c12d464,r0
            cpu.r[0] = mem.read_u32(0x8c12d464).unwrap_or(0);
            // 0x8c12d406: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c12d408: add #76,r4
            cpu.r[4] = cpu.r[4].wrapping_add(76);
            // 0x8c12d40a: jsr @r0 ; delay 0x8c12d40c mov #0,r5
            cpu.pr = 0x8c12d40e;
            cpu.r[5] = 0;
            crate::registry::p_8c0cd048::run(cpu, mem);
            // 0x8c12d40e: mov.l 0x8c12d468,r0
            cpu.r[0] = mem.read_u32(0x8c12d468).unwrap_or(0);
            // 0x8c12d410: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c12d412: add #36,r4
            cpu.r[4] = cpu.r[4].wrapping_add(36);
            // 0x8c12d414: jsr @r0 ; delay 0x8c12d416 mov #0,r5
            cpu.pr = 0x8c12d418;
            cpu.r[5] = 0;
            crate::registry::p_8c07c100::run(cpu, mem);
            // 0x8c12d418: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c12d41a: add #8,r0
            cpu.r[0] = cpu.r[0].wrapping_add(8);
            // 0x8c12d41c: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c12d41e: bt 0x8c12d424
            if cpu.r[0] != 0 {
                // 0x8c12d420: mov.l 0x8c12d46c,r0
                cpu.r[0] = mem.read_u32(0x8c12d46c).unwrap_or(0);
                // 0x8c12d422: mov.l r0,@(8,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]);
            }
        }
        // 0x8c12d424: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c12d426: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c12d428: bf 0x8c12d430
        if (cpu.r[0] as i32) > 0 {
            // 0x8c12d42a: mov.l 0x8c12d470,r0
            cpu.r[0] = mem.read_u32(0x8c12d470).unwrap_or(0);
            // 0x8c12d42c: jsr @r0 ; delay 0x8c12d42e mov r13,r4
            cpu.pr = 0x8c12d430;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c12d430: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c12d432: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c12d434: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c12d436: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c12d438: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c12d43a: rts ; delay 0x8c12d43c mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
