//! 0x8c09f00c — initialise an object: install the class word and the relative
//! vtable/back-pointer offsets, then optionally destroy two embedded
//! sub-objects and optionally free the object itself.
//!
//! r4 = object, r5 = depth (only `exts.w` of it matters).
//!
//! ```text
//! push r14, r13, pr ; r14 = r5 ; r13 = r4
//! tst r13,r13 ; bt out
//! lit = [0x8c09f094] (0x8c1eed70)
//! [r13+8] = lit
//! p = [r13]   ; [p]   = lit + 0x38
//! q = [r13+4] ; [q+4] = lit + 0xd4
//! r = [q]     ; [r+4] = lit + 0xb4
//! [p+24] = (r13+12) - p
//! [r+36] = (r13+40) - r
//! [q+16] = (r13+80) - q       ; bt.s delay slot, runs on both arms
//! if (i16)r14 != 0:
//!     destroy(r13+80, 0)      ; 0x8c0cd048
//!     destroy(r13+40, 0)      ; 0x8c07c100
//!     [r13+12] = 0x8c1e9ad4
//! if (i16)r14 > 0: free(r13)  ; 0x8c01032c
//! out: r0 = r13 ; pop pr, r13 ; rts ; pop r14
//! ```
//!
//! Paths and T:
//!  - `r4 == 0`: only push/pop; T = 1 from `tst r13,r13`; r0 = 0.
//!  - `r4 != 0`, `(i16)r5 == 0`: stores only, no calls; T = 0 from `cmp/pl`.
//!  - `r4 != 0`, `(i16)r5 < 0`: stores + both destroys, no final free; T = 0.
//!  - `r4 != 0`, `(i16)r5 > 0`: everything incl. the final free; T is the
//!    ported `free`'s residue.
//!
//! The `bt` at 0x8c09f078 tests `r13+12`, which is zero only for
//! `r13 == 0xfffffff4`; the header stores before it already reach outside
//! mapped memory for that value, so the oracle never takes that arm with a
//! clean halt. The branch is still transcribed.
//!
//! All three calls go through literal-loaded pointers, so the callees get the
//! original's residue and their ports are invoked directly with the delay-slot
//! argument applied first (r5 = 0 twice, r4 = r13 third).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c09f00c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c09f00e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c09f010: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c09f012: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c09f014: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c09f016: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c09f018: bt 0x8c09f08a
    if cpu.r[13] != 0 {
        // 0x8c09f01a: mov.l 0x8c09f094,r0
        cpu.r[0] = mem.read_u32(0x8c09f094).unwrap_or(0x8c1eed70);
        // 0x8c09f01c: mov r13,r5
        cpu.r[5] = cpu.r[13];
        // 0x8c09f01e: add #12,r5
        cpu.r[5] = cpu.r[5].wrapping_add(12);
        // 0x8c09f020: mov.l r0,@(8,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]);
        // 0x8c09f022: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c09f024: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c09f026: add #56,r1
        cpu.r[1] = cpu.r[1].wrapping_add(56);
        // 0x8c09f028: mov.l r1,@r3
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        // 0x8c09f02a: mov.l 0x8c09f098,r4
        cpu.r[4] = mem.read_u32(0x8c09f098).unwrap_or(0xd4);
        // 0x8c09f02c: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c09f02e: mov.l @(4,r13),r6
        cpu.r[6] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        // 0x8c09f030: add r4,r1
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[4]);
        // 0x8c09f032: mov r13,r3
        cpu.r[3] = cpu.r[13];
        // 0x8c09f034: mov.l r1,@(4,r6)
        let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[1]);
        // 0x8c09f036: mov r13,r4
        cpu.r[4] = cpu.r[13];
        // 0x8c09f038: mov.l @(4,r13),r6
        cpu.r[6] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        // 0x8c09f03a: add #40,r3
        cpu.r[3] = cpu.r[3].wrapping_add(40);
        // 0x8c09f03c: mov.l 0x8c09f09c,r1
        cpu.r[1] = mem.read_u32(0x8c09f09c).unwrap_or(0xb4);
        // 0x8c09f03e: add #80,r4
        cpu.r[4] = cpu.r[4].wrapping_add(80);
        // 0x8c09f040: mov.l @r6,r6
        cpu.r[6] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        // 0x8c09f042: add r1,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
        // 0x8c09f044: exts.w r14,r1
        cpu.r[1] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c09f046: mov.l r0,@(4,r6)
        let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[0]);
        // 0x8c09f048: tst r1,r1
        cpu.set_t(cpu.r[1] == 0);
        // 0x8c09f04a: mov.l @r13,r6
        cpu.r[6] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c09f04c: mov r4,r0
        cpu.r[0] = cpu.r[4];
        // 0x8c09f04e: sub r6,r5
        cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[6]);
        // 0x8c09f050: mov.l r5,@(24,r6)
        let _ = mem.write_u32(cpu.r[6].wrapping_add(24), cpu.r[5]);
        // 0x8c09f052: mov.l @(4,r13),r1
        cpu.r[1] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        // 0x8c09f054: mov.l @r1,r1
        cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        // 0x8c09f056: sub r1,r3
        cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[1]);
        // 0x8c09f058: mov.l r3,@(36,r1)
        let _ = mem.write_u32(cpu.r[1].wrapping_add(36), cpu.r[3]);
        // 0x8c09f05a: mov.l @(4,r13),r1
        cpu.r[1] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        // 0x8c09f05c: sub r1,r0
        cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]);
        // 0x8c09f05e: bt.s 0x8c09f07e ; delay 0x8c09f060: mov.l r0,@(16,r1)
        let _ = mem.write_u32(cpu.r[1].wrapping_add(16), cpu.r[0]);
        if !cpu.t() {
            // 0x8c09f062: mov.l 0x8c09f0a0,r0
            cpu.r[0] = mem.read_u32(0x8c09f0a0).unwrap_or(0x8c0cd048);
            // 0x8c09f064: jsr @r0 ; delay 0x8c09f066: mov #0,r5
            cpu.pr = 0x8c09f068;
            cpu.r[5] = 0;
            crate::registry::p_8c0cd048::run(cpu, mem);
            // 0x8c09f068: mov.l 0x8c09f0a4,r0
            cpu.r[0] = mem.read_u32(0x8c09f0a4).unwrap_or(0x8c07c100);
            // 0x8c09f06a: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c09f06c: add #40,r4
            cpu.r[4] = cpu.r[4].wrapping_add(40);
            // 0x8c09f06e: jsr @r0 ; delay 0x8c09f070: mov #0,r5
            cpu.pr = 0x8c09f072;
            cpu.r[5] = 0;
            crate::registry::p_8c07c100::run(cpu, mem);
            // 0x8c09f072: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c09f074: add #12,r0
            cpu.r[0] = cpu.r[0].wrapping_add(12);
            // 0x8c09f076: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c09f078: bt 0x8c09f07e
            if cpu.r[0] != 0 {
                // 0x8c09f07a: mov.l 0x8c09f0a8,r0
                cpu.r[0] = mem.read_u32(0x8c09f0a8).unwrap_or(0x8c1e9ad4);
                // 0x8c09f07c: mov.l r0,@(12,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[0]);
            }
        }

        // 0x8c09f07e: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c09f080: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c09f082: bf 0x8c09f08a
        if (cpu.r[0] as i32) > 0 {
            // 0x8c09f084: mov.l 0x8c09f0ac,r0
            cpu.r[0] = mem.read_u32(0x8c09f0ac).unwrap_or(0x8c01032c);
            // 0x8c09f086: jsr @r0 ; delay 0x8c09f088: mov r13,r4
            cpu.pr = 0x8c09f08a;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c09f08a: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c09f08c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09f08e: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09f090: rts ; delay 0x8c09f092: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
