//! 0x8c07c100 — destroy/finalise an object at `r4`; `r5` low half selects the
//! ownership depth.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14 ; sts.l pr,@-r15 ; mov r4,r13
//! tst r13,r13 ; bt out
//! r3 = [0x8c07c170] (0x8c1e9774) ; r1 = r13+36 ; [r13+4] = r3
//! r0 = [r13] ; r3 += 32 ; [r0] = r3
//! r3 = [r13] ; r0 = [0x8c07c174] (0x8c0102cc) ; r1 -= r3 ; [r3+24] = r1
//! jsr @r0 ; r4 = [r13+8]    (free)
//! ... repeated for [r13+12], [r13+16], [r13+20], [r13+24], [r13+28]
//! r0 = exts.w r14 ; tst r0,r0 ; bt skip
//! r0 = r13+36 ; tst r0,r0 ; bt skip
//! r0 = [0x8c07c178] (0x8c1e9ad4) ; [r13+36] = r0
//! skip: r0 = exts.w r14 ; cmp/pl r0 ; bf out
//! r0 = [0x8c07c17c] (0x8c01032c) ; jsr @r0 ; r4 = r13  (free the object)
//! out: r0 = r13 ; lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! The six `jsr` calls and the optional seventh go through the thunks
//! 0x8c0102cc and 0x8c01032c (both tail-jump to `free` at 0x8c159fc0), so the
//! callee residue (r0/r1/r3/r4/r5/r6 and T) is whatever the ported `free`
//! leaves. `r2` and `r7`-`r14` are untouched by `free`.
//!
//! Paths:
//!  - `r4 == 0`: only the prologue/epilogue run; T = 1; returns entry `r13`.
//!  - `r4 != 0`, `exts.w r5 == 0`: stores, six frees, no [r13+36] store, no
//!    final free; T = 0 (cmp/pl on zero).
//!  - `r4 != 0`, `exts.w r5 < 0`: stores, six frees, [r13+36] store, no final
//!    free; T = 0.
//!  - `r4 != 0`, `exts.w r5 > 0`: everything incl. the final free; T is the
//!    callee's residue, r0 is overwritten with `r13` at the exit.
//!
//! The `tst r0,r0` on `r13+36` at 0x8c07c150 can only take its branch when
//! `r13 == 0xffffffdc`, whose earlier stores are outside mapped memory, so the
//! oracle never reaches that arm with a clean halt; the branch is still
//! transcribed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07c100: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07c102: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07c104: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07c106: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07c108: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07c10a: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c07c10c: bt 0x8c07c164
    if cpu.r[13] != 0 {
        // 0x8c07c10e: mov.l 0x8c07c170,r3
        cpu.r[3] = mem.read_u32(0x8c07c170).unwrap_or(0); // 0x8c1e9774
        // 0x8c07c110: mov r13,r1
        cpu.r[1] = cpu.r[13];
        // 0x8c07c112: add #36,r1
        cpu.r[1] = cpu.r[1].wrapping_add(36);
        // 0x8c07c114: mov.l r3,@(4,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[3]);
        // 0x8c07c116: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c07c118: add #32,r3
        cpu.r[3] = cpu.r[3].wrapping_add(32);
        // 0x8c07c11a: mov.l r3,@r0
        let _ = mem.write_u32(cpu.r[0], cpu.r[3]);
        // 0x8c07c11c: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c07c11e: mov.l 0x8c07c174,r0
        cpu.r[0] = mem.read_u32(0x8c07c174).unwrap_or(0); // 0x8c0102cc
        // 0x8c07c120: sub r3,r1
        cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[3]);
        // 0x8c07c122: mov.l r1,@(24,r3)
        let _ = mem.write_u32(cpu.r[3].wrapping_add(24), cpu.r[1]);

        // six frees: (jsr address, return address, argument offset)
        const CALLS: [(u32, u32, u32); 6] = [
            (0x8c07c124, 0x8c07c128, 8),
            (0x8c07c12a, 0x8c07c12e, 12),
            (0x8c07c130, 0x8c07c134, 16),
            (0x8c07c136, 0x8c07c13a, 20),
            (0x8c07c13c, 0x8c07c140, 24),
            (0x8c07c142, 0x8c07c146, 28),
        ];
        for (_, ret, offset) in CALLS {
            // mov.l 0x8c07c174,r0 (reloaded after each return)
            cpu.r[0] = mem.read_u32(0x8c07c174).unwrap_or(0); // 0x8c0102cc
            // jsr @r0 ; delay slot: mov.l @(offset,r13),r4
            cpu.pr = ret;
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(offset)).unwrap_or(0);
            crate::registry::p_8c0102cc::run(cpu, mem);
        }

        // 0x8c07c146: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07c148: tst r0,r0
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c07c14a: bt 0x8c07c158
        if cpu.r[0] != 0 {
            // 0x8c07c14c: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c07c14e: add #36,r0
            cpu.r[0] = cpu.r[0].wrapping_add(36);
            // 0x8c07c150: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c07c152: bt 0x8c07c158
            if cpu.r[0] != 0 {
                // 0x8c07c154: mov.l 0x8c07c178,r0
                let v = mem.read_u32(0x8c07c178).unwrap_or(0); // 0x8c1e9ad4
                // 0x8c07c156: mov.l r0,@(36,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(36), v);
            }
        }

        // 0x8c07c158: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07c15a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07c15c: bf 0x8c07c164
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07c15e: mov.l 0x8c07c17c,r0
            cpu.r[0] = mem.read_u32(0x8c07c17c).unwrap_or(0); // 0x8c01032c
            // 0x8c07c160: jsr @r0 ; delay slot 0x8c07c162: mov r13,r4
            cpu.pr = 0x8c07c164;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c07c164: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c07c166: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07c168: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07c16a: rts ; delay slot 0x8c07c16c: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
