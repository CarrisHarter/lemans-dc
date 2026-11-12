//! 0x8c07cdd4 — destroy/finalise an object at `r4`; `r5` low half selects the
//! ownership depth. Same shape as 0x8c07c100, but four payload frees at
//! `r4+32..r4+44` and the class word at `r4+48` instead of six and `+36`.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14 ; sts.l pr,@-r15 ; mov r4,r13
//! tst r13,r13 ; bt out
//! r3 = [0x8c07ce38] (0x8c1e98cc) ; r1 = r13+48 ; [r13+4] = r3
//! r0 = [r13] ; r3 += 64 ; [r0] = r3
//! r3 = [r13] ; r0 = [0x8c07ce3c] (0x8c0102cc) ; r1 -= r3 ; [r3+24] = r1
//! jsr @r0 ; r4 = [r13+32]    (free)
//! ... repeated for [r13+36], [r13+40], [r13+44]
//! r0 = exts.w r14 ; tst r0,r0 ; bt skip
//! r0 = r13+48 ; tst r0,r0 ; bt skip
//! r0 = [0x8c07ce40] (0x8c1e9ad4) ; [r13+48] = r0
//! skip: r0 = exts.w r14 ; cmp/pl r0 ; bf out
//! r0 = [0x8c07ce44] (0x8c01032c) ; jsr @r0 ; r4 = r13  (free the object)
//! out: r0 = r13 ; lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! The four `jsr` calls and the optional fifth go through the thunks
//! 0x8c0102cc and 0x8c01032c (both tail-jump to `free` at 0x8c159fc0), so the
//! callee residue (r0/r1/r3/r4/r5/r6 and T) is whatever the ported `free`
//! leaves.
//!
//! Paths:
//!  - `r4 == 0`: only the prologue/epilogue run; T = 1; returns entry `r13`.
//!  - `r4 != 0`, `exts.w r5 == 0`: stores, four frees, no [r13+48] store, no
//!    final free; T = 0 (cmp/pl on zero).
//!  - `r4 != 0`, `exts.w r5 < 0`: stores, four frees, [r13+48] store, no final
//!    free; T = 0.
//!  - `r4 != 0`, `exts.w r5 > 0`: everything incl. the final free; T is the
//!    callee's residue, r0 is overwritten with `r13` at the exit.
//!
//! The `tst r0,r0` on `r13+48` at 0x8c07ce18 can only take its branch when
//! `r13 == 0xffffffd0`, whose earlier stores are outside mapped memory, so the
//! oracle never reaches that arm with a clean halt; the branch is still
//! transcribed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07cdd4: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07cdd6: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07cdd8: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07cdda: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07cddc: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07cdde: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c07cde0: bt 0x8c07ce2c
    if cpu.r[13] != 0 {
        // 0x8c07cde2: mov.l 0x8c07ce38,r3
        cpu.r[3] = mem.read_u32(0x8c07ce38).unwrap_or(0); // 0x8c1e98cc
        // 0x8c07cde4: mov r13,r1
        cpu.r[1] = cpu.r[13];
        // 0x8c07cde6: add #48,r1
        cpu.r[1] = cpu.r[1].wrapping_add(48);
        // 0x8c07cde8: mov.l r3,@(4,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[3]);
        // 0x8c07cdea: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c07cdec: add #64,r3
        cpu.r[3] = cpu.r[3].wrapping_add(64);
        // 0x8c07cdee: mov.l r3,@r0
        let _ = mem.write_u32(cpu.r[0], cpu.r[3]);
        // 0x8c07cdf0: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c07cdf2: mov.l 0x8c07ce3c,r0
        cpu.r[0] = mem.read_u32(0x8c07ce3c).unwrap_or(0); // 0x8c0102cc
        // 0x8c07cdf4: sub r3,r1
        cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[3]);
        // 0x8c07cdf6: mov.l r1,@(24,r3)
        let _ = mem.write_u32(cpu.r[3].wrapping_add(24), cpu.r[1]);

        // four frees: (return address, argument offset)
        const CALLS: [(u32, u32); 4] = [
            (0x8c07cdfc, 32),
            (0x8c07ce02, 36),
            (0x8c07ce08, 40),
            (0x8c07ce0e, 44),
        ];
        for (ret, offset) in CALLS {
            // mov.l 0x8c07ce3c,r0 (reloaded after each return)
            cpu.r[0] = mem.read_u32(0x8c07ce3c).unwrap_or(0); // 0x8c0102cc
            // jsr @r0 ; delay slot: mov.l @(offset,r13),r4
            cpu.pr = ret;
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(offset)).unwrap_or(0);
            crate::registry::p_8c0102cc::run(cpu, mem);
        }

        // 0x8c07ce0e: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07ce10: tst r0,r0
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c07ce12: bt 0x8c07ce20
        if cpu.r[0] != 0 {
            // 0x8c07ce14: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c07ce16: add #48,r0
            cpu.r[0] = cpu.r[0].wrapping_add(48);
            // 0x8c07ce18: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c07ce1a: bt 0x8c07ce20
            if cpu.r[0] != 0 {
                // 0x8c07ce1c: mov.l 0x8c07ce40,r0
                let v = mem.read_u32(0x8c07ce40).unwrap_or(0); // 0x8c1e9ad4
                // 0x8c07ce1e: mov.l r0,@(48,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(48), v);
            }
        }

        // 0x8c07ce20: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07ce22: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07ce24: bf 0x8c07ce2c
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07ce26: mov.l 0x8c07ce44,r0
            cpu.r[0] = mem.read_u32(0x8c07ce44).unwrap_or(0); // 0x8c01032c
            // 0x8c07ce28: jsr @r0 ; delay slot 0x8c07ce2a: mov r13,r4
            cpu.pr = 0x8c07ce2c;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c07ce2c: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c07ce2e: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07ce30: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07ce32: rts ; delay slot 0x8c07ce34: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
