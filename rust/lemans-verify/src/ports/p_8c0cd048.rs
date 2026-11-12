//! 0x8c0cd048 — tear down a scene/object descriptor: install the vtable and
//! class words, free the two owned pointers, optionally destroy the embedded
//! sub-object through 0x8c07c100, and optionally free the descriptor itself.
//!
//! r4 = descriptor, r5 = depth (only `exts.w` of it matters).
//!
//! ```text
//! push r14, r13, pr ; r14 = r5 ; r13 = r4
//! tst r13,r13 ; bt out
//! [r13+4] = 0x8c1ebefc
//! p = [r13] ; [p+4] = 0x8c1ebf90
//! q = [p]   ; [q]   = 0x8c1ebf14
//! [q+24] = (r13+16) - q ; [p+36] = (r13+44) - p
//! free([r13+8]) ; free([r13+12])
//! if (short)r14 != 0:
//!     destroy(r13+44, 0)          ; 0x8c07c100
//!     [r13+16] = 0x8c1e9ad4       ; `tst r0,r0` on r13+16 is never zero here
//! if (short)r14 > 0: free(r13)
//! out: r0 = r13 ; pop pr, r13 ; rts ; pop r14
//! ```
//!
//! Paths and T:
//!  - `r4 == 0`: only push/pop; T = 1 from `tst r13,r13`.
//!  - `r4 != 0`, `(i16)r5 == 0`: no destroy, no final free; T = 0 from
//!    `cmp/pl` on zero.
//!  - `r4 != 0`, `(i16)r5 < 0`: destroy, no final free; T = 0.
//!  - `r4 != 0`, `(i16)r5 > 0`: destroy and final free; T is `free`'s residue.
//!
//! The `tst r0,r0` guarding the [r13+16] store tests `r13+16`, which can only be
//! zero for `r13 == 0xfffffff0` (already excluded by the earlier `bt`), so the
//! store always happens on the destroy path; the branch is still transcribed.
//!
//! Both free sites go through the 0x8c0102cc / 0x8c01032c thunks, so the
//! callee residue in r0/r1/r3-r6 and T is the ported `free`'s.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0cd048: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0cd04a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0cd04c: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0cd04e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0cd050: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0cd052: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0cd054: bt 0x8c0cd0b6
    if cpu.r[13] != 0 {
        // 0x8c0cd056: mov.l 0x8c0cd0c0,r4
        cpu.r[4] = mem.read_u32(0x8c0cd0c0).unwrap_or(0x8c1ebefc);
        // 0x8c0cd058: mov.l r4,@(4,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[4]);
        // 0x8c0cd05a: mov r4,r0
        cpu.r[0] = cpu.r[4];
        // 0x8c0cd05c: mov.l 0x8c0cd0c4,r1
        cpu.r[1] = mem.read_u32(0x8c0cd0c4).unwrap_or(0x94);
        // 0x8c0cd05e: add #24,r4
        cpu.r[4] = cpu.r[4].wrapping_add(24);
        // 0x8c0cd060: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cd062: add r1,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
        // 0x8c0cd064: mov r13,r1
        cpu.r[1] = cpu.r[13];
        // 0x8c0cd066: mov.l r0,@(4,r3)
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
        // 0x8c0cd068: add #44,r1
        cpu.r[1] = cpu.r[1].wrapping_add(44);
        // 0x8c0cd06a: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cd06c: mov r13,r3
        cpu.r[3] = cpu.r[13];
        // 0x8c0cd06e: add #16,r3
        cpu.r[3] = cpu.r[3].wrapping_add(16);
        // 0x8c0cd070: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c0cd072: mov.l r4,@r0
        let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
        // 0x8c0cd074: mov.l @r13,r4
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cd076: mov.l 0x8c0cd0c8,r0
        cpu.r[0] = mem.read_u32(0x8c0cd0c8).unwrap_or(0x8c0102cc);
        // 0x8c0cd078: mov.l @r4,r4
        cpu.r[4] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        // 0x8c0cd07a: sub r4,r3
        cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[4]);
        // 0x8c0cd07c: mov.l r3,@(24,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[3]);
        // 0x8c0cd07e: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cd080: sub r3,r1
        cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[3]);
        // 0x8c0cd082: mov.l r1,@(36,r3)
        let _ = mem.write_u32(cpu.r[3].wrapping_add(36), cpu.r[1]);

        // 0x8c0cd084: jsr @r0 ; delay 0x8c0cd086 mov.l @(8,r13),r4
        cpu.pr = 0x8c0cd088;
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
        crate::registry::p_8c0102cc::run(cpu, mem);
        // 0x8c0cd088: mov.l 0x8c0cd0c8,r0
        cpu.r[0] = mem.read_u32(0x8c0cd0c8).unwrap_or(0x8c0102cc);
        // 0x8c0cd08a: jsr @r0 ; delay 0x8c0cd08c mov.l @(12,r13),r4
        cpu.pr = 0x8c0cd08e;
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
        crate::registry::p_8c0102cc::run(cpu, mem);

        // 0x8c0cd08e: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0cd090: tst r0,r0
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c0cd092: bt 0x8c0cd0aa
        if cpu.r[0] != 0 {
            // 0x8c0cd094: mov.l 0x8c0cd0cc,r0
            cpu.r[0] = mem.read_u32(0x8c0cd0cc).unwrap_or(0x8c07c100);
            // 0x8c0cd096: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c0cd098: add #44,r4
            cpu.r[4] = cpu.r[4].wrapping_add(44);
            // 0x8c0cd09a: jsr @r0 ; delay 0x8c0cd09c mov #0,r5
            cpu.pr = 0x8c0cd09e;
            cpu.r[5] = 0;
            crate::registry::p_8c07c100::run(cpu, mem);

            // 0x8c0cd09e: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c0cd0a0: add #16,r0
            cpu.r[0] = cpu.r[0].wrapping_add(16);
            // 0x8c0cd0a2: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c0cd0a4: bt 0x8c0cd0aa
            if cpu.r[0] != 0 {
                // 0x8c0cd0a6: mov.l 0x8c0cd0d0,r0
                cpu.r[0] = mem.read_u32(0x8c0cd0d0).unwrap_or(0x8c1e9ad4);
                // 0x8c0cd0a8: mov.l r0,@(16,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[0]);
            }
        }

        // 0x8c0cd0aa: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0cd0ac: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0cd0ae: bf 0x8c0cd0b6
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0cd0b0: mov.l 0x8c0cd0d4,r0
            cpu.r[0] = mem.read_u32(0x8c0cd0d4).unwrap_or(0x8c01032c);
            // 0x8c0cd0b2: jsr @r0 ; delay 0x8c0cd0b4 mov r13,r4
            cpu.pr = 0x8c0cd0b6;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c0cd0b6: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0cd0b8: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0cd0ba: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0cd0bc: rts ; delay 0x8c0cd0be mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
